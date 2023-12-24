# Food Allergy Profile Service

This repository contains a Canister for managing food allergy profiles. The service provides functionalities to add, delete, and retrieve information about food allergy profiles. It includes features like listing all profiles, searching by allergy, product recommendation, user ID, and filtering by updated timestamps.

## Data Structures

### `Error`
Represents error types, including a `NotFound` variant with a descriptive message.

### `FoodAllergyProfile`
A struct representing a food allergy profile with attributes such as ID, creation and update timestamps, user ID, allergies, and product recommendations.

### `FoodAllergyUpdatePayload`
A payload structure for updating food allergy profiles, including allergies and product recommendations.

### `Result`
A variant representing the result of operations. Includes an `Ok` variant with a `FoodAllergyProfile` or an `Err` variant with an `Error`.

## Service Functions

1. **add_food_allergy_profile:**
   - Adds a new food allergy profile with automatically generated ID and creation timestamp.

2. **delete_food_allergy_profile:**
   - Deletes a food allergy profile based on the provided ID.

3. **get_all_food_allergy_profiles:**
   - Retrieves a list of all stored food allergy profiles.

4. **get_food_allergy_profile:**
   - Retrieves detailed information about a specific food allergy profile based on its ID.

5. **get_food_allergy_profiles_by_allergy:**
   - Retrieves food allergy profiles that match a given allergy.

6. **get_food_allergy_profiles_by_product_recommendation:**
   - Retrieves food allergy profiles that match a given product recommendation.

7. **get_food_allergy_profiles_by_user_id:**
   - Retrieves food allergy profiles associated with a specific user ID.

8. **get_food_allergy_profiles_updated_after:**
   - Retrieves food allergy profiles updated after a specified timestamp.

9. **update_food_allergy_profile:**
   - Updates a food allergy profile based on the provided ID and `FoodAllergyUpdatePayload`.

## Candid Interface

The canister exports its Candid interface definitions using the `ic_cdk::export_candid!()` macro.

## Error Handling

Errors are represented using the `Error` enum, which includes a `NotFound` variant with a descriptive message.

Feel free to explore and integrate this canister into your Internet Computer project for efficient food allergy profile management!