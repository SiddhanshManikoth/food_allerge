#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;
// ... (existing imports and types)

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct FoodAllergyProfile {
    id: u64,
    user_id: u64,
    allergies: Vec<String>,
    product_recommendations: Vec<String>,
    created_at: u64,
    updated_at: Option<u64>,
}

// Implementing Storable and BoundedStorable traits for FoodAllergyProfile
impl Storable for FoodAllergyProfile {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for FoodAllergyProfile {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// ... (existing thread-local variables and payload structure)

// New thread-local variables for our Food Allergy app

thread_local! {
    static ALLERGY_MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ALLERGY_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(ALLERGY_MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter for food allergy profiles")
    );

    static ALLERGY_STORAGE: RefCell<StableBTreeMap<u64, FoodAllergyProfile, Memory>> =
        RefCell::new(StableBTreeMap::init(
            ALLERGY_MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
}

// Helper method to perform insert for FoodAllergyProfile
fn do_insert_food_allergy_profile(profile: &FoodAllergyProfile) {
    ALLERGY_STORAGE.with(|service| service.borrow_mut().insert(profile.id, profile.clone()));
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct FoodAllergyUpdatePayload {
    allergies: Vec<String>,
    product_recommendations: Vec<String>,
}

// ... (existing imports and types)

// 2.7 Managing Food Allergy Profiles
// In this section, we'll implement the core logic for managing food allergy profiles within our canister.

// 2.7.1 get_food_allergy_profile Function:
#[ic_cdk::query]
fn get_food_allergy_profile(id: u64) -> Result<FoodAllergyProfile, Error> {
    match _get_food_allergy_profile(&id) {
        Some(profile) => Ok(profile),
        None => Err(Error::NotFound {
            msg: format!("a food allergy profile with id={} not found", id),
        }),
    }
}

// 2.7.2 _get_food_allergy_profile Function:
fn _get_food_allergy_profile(id: &u64) -> Option<FoodAllergyProfile> {
    ALLERGY_STORAGE.with(|s| s.borrow().get(id))
}


// 2.7.3 add_food_allergy_profile Function:
#[ic_cdk::update]
fn add_food_allergy_profile(profile: FoodAllergyUpdatePayload) -> Option<FoodAllergyProfile> {
    let id = ALLERGY_ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter for food allergy profiles");

    let user_id = ALLERGY_ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment user id counter for food allergy profiles");

    let timestamp = time();
    let food_allergy_profile = FoodAllergyProfile {
        id,
        user_id,
        allergies: profile.allergies,
        product_recommendations: profile.product_recommendations,
        created_at: timestamp,
        updated_at: None,
    };
    do_insert_food_allergy_profile(&food_allergy_profile);
    Some(food_allergy_profile)
}

// 2.7.4 update_food_allergy_profile Function:
#[ic_cdk::update]
fn update_food_allergy_profile(
    id: u64,
    payload: FoodAllergyUpdatePayload,
) -> Result<FoodAllergyProfile, Error> {
    match ALLERGY_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut food_allergy_profile) => {
            food_allergy_profile.allergies = payload.allergies;
            food_allergy_profile.product_recommendations = payload.product_recommendations;
            food_allergy_profile.updated_at = Some(time());
            do_insert_food_allergy_profile(&food_allergy_profile);
            Ok(food_allergy_profile)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update a food allergy profile with id={}. profile not found",
                id
            ),
        }),
    }
}

// 2.7.5 delete_food_allergy_profile Function:
#[ic_cdk::update]
fn delete_food_allergy_profile(id: u64) -> Result<FoodAllergyProfile, Error> {
    match ALLERGY_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(food_allergy_profile) => Ok(food_allergy_profile),
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't delete a food allergy profile with id={}. profile not found.",
                id
            ),
        }),
    }
}

// 2.7.6 get_all_food_allergy_profiles Function:
#[ic_cdk::query]
fn get_all_food_allergy_profiles() -> Vec<FoodAllergyProfile> {
    ALLERGY_STORAGE.with(|service| {
        let storage = service.borrow_mut();
        storage.iter().map(|(_, item)| item.clone()).collect()
    })
}

#[ic_cdk::query]
fn get_food_allergy_profiles_updated_after(timestamp: u64) -> Vec<FoodAllergyProfile> {
    ALLERGY_STORAGE.with(|service| {
        let borrow = service.borrow();
        borrow
            .iter()
            .filter_map(|(_, profile)| {
                if let Some(updated_at) = profile.updated_at {
                    if updated_at > timestamp {
                        Some(profile.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    })
}

#[ic_cdk::query]
fn get_food_allergy_profiles_by_user_id(user_id: u64) -> Vec<FoodAllergyProfile> {
    ALLERGY_STORAGE.with(|service| {
        let borrow = service.borrow();
        borrow
            .iter()
            .filter_map(|(_, profile)| {
                if profile.user_id == user_id {
                    Some(profile.clone())
                } else {
                    None
                }
            })
            .collect()
    })
}

#[ic_cdk::query]
fn get_food_allergy_profiles_by_product_recommendation(product: String) -> Vec<FoodAllergyProfile> {
    ALLERGY_STORAGE.with(|service| {
        let borrow = service.borrow();
        borrow
            .iter()
            .filter_map(|(_, profile)| {
                if profile.product_recommendations.contains(&product) {
                    Some(profile.clone())
                } else {
                    None
                }
            })
            .collect()
    })
}

#[ic_cdk::query]
fn get_food_allergy_profiles_by_allergy(allergy: String) -> Vec<FoodAllergyProfile> {
    ALLERGY_STORAGE.with(|service| {
        let borrow = service.borrow();
        borrow
            .iter()
            .filter_map(|(_, profile)| {
                if profile.allergies.contains(&allergy) {
                    Some(profile.clone())
                } else {
                    None
                }
            })
            .collect()
    })
}

// 2.7.7 enum Error:
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
}

// To generate the Candid interface definitions for our canister
ic_cdk::export_candid!();
