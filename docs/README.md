# Requirements

## Functional Requirements
Develop an online e-commerce store **selling fruit**, which contains the following **features**:
1. - [ ] User sign-up and login.
2. - [ ] Browse the following products
    1. Apples
    1. Bananas
    1. Pears
    1. Oranges
1. - [ ] Add items to your cart
    1. Adjust quantity.
    1. Delete items from the cart.
1. - [ ] Checkout your cart.
    1. Mocked purchase (a payment gateway is not required, but a route must exist in the backend validating the payment).
    1. An address does not need to be entered.
1. - [ ] Users must be able to return to their cart after closing the browser, and see the previous items that were added.

### Checkout Rules
1. If 7 or more apples are added to the cart, a 10% discount is applied to all apples.
1. For each set of 4 pears and 2 bananas, a 30% discount is applied, to each set.
    1. These sets must be added to their own cart item entry.
    1. If pears or bananas already exist in the cart, this discount must be recalculated when new pears or bananas are added.
1. A coupon code can be used to get a 30% discount on oranges, if applied to the cart, otherwise oranges are full price.
    1. Can only be applied once.
    1. Has a configurable expiry timeout (10 seconds default) once generated.

## Non-Functional Requirements
1. Architecture diagrams.
1. Single-page frontend app with latest technology (cannot use an existing online stores such as *Prestashop*).
1. Backend RESTful web service written in **Go**.
1. Use up-to-date technology.
