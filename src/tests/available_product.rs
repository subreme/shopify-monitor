// This document contains 50 tests for a function that has been
// modified, therefore they no longer work.

// #[cfg(test)]
// mod available_product_test {
//     use crate::{
//         monitor::{available_product, AvailableVariant, MinimalVariant},
//         products::{Product, Variant},
//     };

//     // These first three functions are not tests: they are used to
//     // generate fake data using only the necessary parameters, in order
//     // to avoid clutter so that the tests are easier to read.

//     // This function generates a `Product` using a vector of `Variants`,
//     // which in turn are generated using the `v()` function. Although
//     // all tests in this module currently use two variants, I decided
//     // not to merge the two functions and hard-code the number of
//     // variants, however I decided not to, as that might change in the
//     // future and I wanted the use of these functions to be consistent.
//     fn p(variants: Vec<Variant>) -> Product {
//         Product {
//             id: 0,
//             title: "".into(),
//             handle: "".into(),
//             body_html: "".into(),
//             published_at: "".into(),
//             created_at: "".into(),
//             updated_at: "".into(),
//             vendor: "".into(),
//             product_type: "".into(),
//             tags: vec![],

//             // I know the field name isn't necessary in this case, but I
//             // like consistency.
//             variants: variants,
//             images: vec![],
//             options: vec![],
//         }
//     }

//     fn v(id: u64, available: bool) -> Variant {
//         Variant {
//             id: id,
//             title: "".into(),
//             option1: "".into(),
//             option2: None,
//             option3: None,
//             sku: "".into(),
//             requires_shipping: true,
//             taxable: true,
//             featured_image: None,
//             available: available,
//             price: "".into(),
//             grams: 0,
//             compare_at_price: None,
//             position: 0,
//             product_id: 0,
//             created_at: "".into(),
//             updated_at: "".into(),
//         }
//     }

//     fn mv(id: u64, available: bool) -> MinimalVariant {
//         MinimalVariant { id, available }
//     }

//     fn av(id: u64) -> AvailableVariant {
//         AvailableVariant {
//             name: "".into(),
//             id: id,
//         }
//     }

//     // The actual tests begin here.

//     // In this sub-module, all current variants are available.
//     mod all_avbl_curr {
//         use super::*;
//         #[test]
//         fn all_avbl_prev_same() {
//             let current = p(vec![v(0, true), v(1, true)]);
//             let previous = vec![mv(0, true), mv(1, true)];
//             let available = vec![];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn all_avbl_prev_diff() {
//             let current = p(vec![v(0, true), v(1, true)]);
//             let previous = vec![mv(1, true), mv(2, true)];
//             let available = vec![av(0)];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn some_avbl_prev_same() {
//             let current = p(vec![v(0, true), v(1, true)]);
//             let previous = vec![mv(0, true), mv(1, false)];
//             let available = vec![av(1)];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn some_avbl_prev_same_2() {
//             let current = p(vec![v(0, true), v(1, true)]);
//             let previous = vec![mv(0, false), mv(1, true)];
//             let available = vec![av(0)];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn some_avbl_prev_diff() {
//             let current = p(vec![v(0, true), v(1, true)]);
//             let previous = vec![mv(1, true), mv(2, false)];
//             let available = vec![av(0)];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn some_avbl_prev_diff_2() {
//             let current = p(vec![v(0, true), v(1, true)]);
//             let previous = vec![mv(1, false), mv(2, true)];
//             let available = vec![av(0), av(1)];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn no_avbl_prev_same() {
//             let current = p(vec![v(0, true), v(1, true)]);
//             let previous = vec![mv(0, false), mv(1, false)];
//             let available = vec![av(0), av(1)];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn no_avbl_prev_diff() {
//             let current = p(vec![v(0, true), v(1, true)]);
//             let previous = vec![mv(1, false), mv(2, false)];
//             let available = vec![av(0), av(1)];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn empty_prev() {
//             let current = p(vec![v(0, true), v(1, true)]);
//             let previous = vec![];
//             let available = vec![av(0), av(1)];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn no_prev() {
//             let current = p(vec![v(0, true), v(1, true)]);
//             // let previous = vec![];
//             let available = vec![av(0), av(1)];

//             assert_eq!(available_product(&current, None).variants, available);
//         }
//     }

//     // In this sub-module, some current variants are available.
//     mod some_avbl_curr {
//         use super::*;

//         #[test]
//         fn all_avbl_prev_same() {
//             let current = p(vec![v(0, true), v(1, false)]);
//             let previous = vec![mv(0, true), mv(1, true)];
//             let available = vec![];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn all_avbl_prev_same_2() {
//             let current = p(vec![v(0, false), v(1, true)]);
//             let previous = vec![mv(0, true), mv(1, true)];
//             let available = vec![];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn all_avbl_prev_diff() {
//             let current = p(vec![v(0, true), v(1, false)]);
//             let previous = vec![mv(1, true), mv(2, true)];
//             let available = vec![av(0)];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn all_avbl_prev_diff_2() {
//             let current = p(vec![v(0, false), v(1, true)]);
//             let previous = vec![mv(1, true), mv(2, true)];
//             let available = vec![];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn some_avbl_prev_same() {
//             let current = p(vec![v(0, true), v(1, false)]);
//             let previous = vec![mv(0, true), mv(1, false)];
//             let available = vec![];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn some_avbl_prev_same_2() {
//             let current = p(vec![v(0, false), v(1, true)]);
//             let previous = vec![mv(0, true), mv(1, false)];
//             let available = vec![av(1)];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn some_avbl_prev_same_3() {
//             let current = p(vec![v(0, true), v(1, false)]);
//             let previous = vec![mv(0, false), mv(1, true)];
//             let available = vec![av(0)];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn some_avbl_prev_same_4() {
//             let current = p(vec![v(0, false), v(1, true)]);
//             let previous = vec![mv(0, false), mv(1, true)];
//             let available = vec![];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn some_avbl_prev_diff() {
//             let current = p(vec![v(0, true), v(1, false)]);
//             let previous = vec![mv(1, true), mv(2, false)];
//             let available = vec![av(0)];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn some_avbl_prev_diff_2() {
//             let current = p(vec![v(0, false), v(1, true)]);
//             let previous = vec![mv(1, true), mv(2, false)];
//             let available = vec![];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn some_avbl_prev_diff_3() {
//             let current = p(vec![v(0, true), v(1, false)]);
//             let previous = vec![mv(1, false), mv(2, true)];
//             let available = vec![av(0)];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn some_avbl_prev_diff_4() {
//             let current = p(vec![v(0, false), v(1, true)]);
//             let previous = vec![mv(1, false), mv(2, true)];
//             let available = vec![av(1)];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn no_avbl_prev_same() {
//             let current = p(vec![v(0, true), v(1, false)]);
//             let previous = vec![mv(0, false), mv(1, false)];
//             let available = vec![av(0)];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn no_avbl_prev_same_2() {
//             let current = p(vec![v(0, false), v(1, true)]);
//             let previous = vec![mv(0, false), mv(1, false)];
//             let available = vec![av(1)];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn no_avbl_prev_diff() {
//             let current = p(vec![v(0, true), v(1, false)]);
//             let previous = vec![mv(1, false), mv(2, false)];
//             let available = vec![av(0)];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn no_avbl_prev_diff_2() {
//             let current = p(vec![v(0, false), v(1, true)]);
//             let previous = vec![mv(1, false), mv(2, false)];
//             let available = vec![av(1)];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn empty_prev() {
//             let current = p(vec![v(0, true), v(1, false)]);
//             let previous = vec![];
//             let available = vec![av(0)];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn empty_prev_2() {
//             let current = p(vec![v(0, false), v(1, true)]);
//             let previous = vec![];
//             let available = vec![av(1)];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn no_prev() {
//             let current = p(vec![v(0, true), v(1, false)]);
//             // let previous = vec![];
//             let available = vec![av(0)];

//             assert_eq!(available_product(&current, None).variants, available);
//         }

//         #[test]
//         fn no_prev_2() {
//             let current = p(vec![v(0, false), v(1, true)]);
//             // let previous = vec![];
//             let available = vec![av(1)];

//             assert_eq!(available_product(&current, None).variants, available);
//         }
//     }

//     // In this sub-module, no current variants are available.
//     mod no_avbl_curr {
//         use super::*;
//         #[test]
//         fn all_avbl_prev_same() {
//             let current = p(vec![v(0, false), v(1, false)]);
//             let previous = vec![mv(0, true), mv(1, true)];
//             let available = vec![];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn all_avbl_prev_diff() {
//             let current = p(vec![v(0, false), v(1, false)]);
//             let previous = vec![mv(1, true), mv(2, true)];
//             let available = vec![];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn some_avbl_prev_same() {
//             let current = p(vec![v(0, false), v(1, false)]);
//             let previous = vec![mv(0, true), mv(1, false)];
//             let available = vec![];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn some_avbl_prev_same_2() {
//             let current = p(vec![v(0, false), v(1, false)]);
//             let previous = vec![mv(0, false), mv(1, true)];
//             let available = vec![];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn some_avbl_prev_diff() {
//             let current = p(vec![v(0, false), v(1, false)]);
//             let previous = vec![mv(1, true), mv(2, false)];
//             let available = vec![];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn some_avbl_prev_diff_2() {
//             let current = p(vec![v(0, false), v(1, false)]);
//             let previous = vec![mv(1, false), mv(2, true)];
//             let available = vec![];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn no_avbl_prev_same() {
//             let current = p(vec![v(0, false), v(1, false)]);
//             let previous = vec![mv(0, false), mv(1, false)];
//             let available = vec![];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn no_avbl_prev_diff() {
//             let current = p(vec![v(0, false), v(1, false)]);
//             let previous = vec![mv(1, false), mv(2, false)];
//             let available = vec![];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn empty_prev() {
//             let current = p(vec![v(0, false), v(1, false)]);
//             let previous = vec![];
//             let available = vec![];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn no_prev() {
//             let current = p(vec![v(0, false), v(1, false)]);
//             // let previous = vec![];
//             let available = vec![];

//             assert_eq!(available_product(&current, None).variants, available);
//         }
//     }

//     // In this sub-module, no there aren't any variants in the current
//     // product. It isn't named `no_curr`, as the `no_` prefix was used
//     // to indicate the value `None` in `available_product`'s second
//     // (`prev`) argument, which is a `Option<&Vec<MinimalVariant>>`.
//     mod empty_curr {
//         use super::*;
//         #[test]
//         fn all_avbl_prev_same() {
//             let current = p(vec![v(0, false), v(1, false)]);
//             let previous = vec![mv(0, true), mv(1, true)];
//             let available = vec![];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn all_avbl_prev_diff() {
//             let current = p(vec![]);
//             let previous = vec![mv(1, true), mv(2, true)];
//             let available = vec![];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn some_avbl_prev_same() {
//             let current = p(vec![]);
//             let previous = vec![mv(0, true), mv(1, false)];
//             let available = vec![];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn some_avbl_prev_same_2() {
//             let current = p(vec![]);
//             let previous = vec![mv(0, false), mv(1, true)];
//             let available = vec![];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn some_avbl_prev_diff() {
//             let current = p(vec![]);
//             let previous = vec![mv(1, true), mv(2, false)];
//             let available = vec![];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn some_avbl_prev_diff_2() {
//             let current = p(vec![]);
//             let previous = vec![mv(1, false), mv(2, true)];
//             let available = vec![];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn no_avbl_prev_same() {
//             let current = p(vec![]);
//             let previous = vec![mv(0, false), mv(1, false)];
//             let available = vec![];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn no_avbl_prev_diff() {
//             let current = p(vec![]);
//             let previous = vec![mv(1, false), mv(2, false)];
//             let available = vec![];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn empty_prev() {
//             let current = p(vec![]);
//             let previous = vec![];
//             let available = vec![];

//             assert_eq!(
//                 available_product(&current, Some(&previous)).variants,
//                 available
//             );
//         }

//         #[test]
//         fn no_prev() {
//             let current = p(vec![v(0, false), v(1, false)]);
//             // let previous = vec![];
//             let available = vec![];

//             assert_eq!(available_product(&current, None).variants, available);
//         }
//     }
// }
