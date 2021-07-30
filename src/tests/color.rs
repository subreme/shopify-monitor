// This document contains 16 tests for a function that has been
// temporarily removed, therefore they no longer work.

// #[cfg(test)]
// mod color_test {
//     use crate::stores::color;

//     #[test]
//     fn valid_event_valid_server() {
//         assert_eq!(
//             color(
//                 Some("01E240".into()),
//                 Some("BC614E".into()),
//                 "".into(),
//                 Some("".into()),
//                 0
//             ),
//             Some(123456)
//         );
//     }

//     #[test]
//     fn valid_event_hashtag_server() {
//         assert_eq!(
//             color(
//                 Some("01E240".into()),
//                 Some("#BC614E".into()),
//                 "".into(),
//                 Some("".into()),
//                 0
//             ),
//             Some(123456)
//         );
//     }

//     #[test]
//     fn valid_event_invalid_server() {
//         assert_eq!(
//             color(
//                 Some("01E240".into()),
//                 Some("XXXXXX".into()),
//                 "".into(),
//                 Some("".into()),
//                 0
//             ),
//             Some(123456)
//         );
//     }

//     #[test]
//     fn valid_event_no_server() {
//         assert_eq!(
//             color(Some("01E240".into()), None, "".into(), Some("".into()), 0),
//             Some(123456)
//         );
//     }

//     #[test]
//     fn hashtag_event_valid_server() {
//         assert_eq!(
//             color(
//                 Some("#01E240".into()),
//                 Some("BC614E".into()),
//                 "".into(),
//                 Some("".into()),
//                 0
//             ),
//             Some(123456)
//         );
//     }

//     #[test]
//     fn hashtag_event_hashtag_server() {
//         assert_eq!(
//             color(
//                 Some("01E240".into()),
//                 Some("#BC614E".into()),
//                 "".into(),
//                 Some("".into()),
//                 0
//             ),
//             Some(123456)
//         );
//     }

//     #[test]
//     fn hashtag_event_invalid_server() {
//         assert_eq!(
//             color(
//                 Some("01E240".into()),
//                 Some("XXXXXX".into()),
//                 "".into(),
//                 Some("".into()),
//                 0
//             ),
//             Some(123456)
//         );
//     }

//     #[test]
//     fn hashtag_event_no_server() {
//         assert_eq!(
//             color(Some("01E240".into()), None, "".into(), Some("".into()), 0),
//             Some(123456)
//         );
//     }

//     #[test]
//     fn invalid_event_valid_server() {
//         assert_eq!(
//             color(
//                 Some("XXXXXX".into()),
//                 Some("BC614E".into()),
//                 "".into(),
//                 Some("".into()),
//                 0
//             ),
//             Some(12345678)
//         );
//     }

//     #[test]
//     fn invalid_event_hashtag_server() {
//         assert_eq!(
//             color(
//                 Some("XXXXXX".into()),
//                 Some("#BC614E".into()),
//                 "".into(),
//                 Some("".into()),
//                 0
//             ),
//             Some(12345678)
//         );
//     }

//     #[test]
//     fn invalid_event_invalid_server() {
//         assert_eq!(
//             color(
//                 Some("XXXXXX".into()),
//                 Some("XXXXXX".into()),
//                 "".into(),
//                 Some("".into()),
//                 0
//             ),
//             None
//         );
//     }

//     #[test]
//     fn invalid_event_no_server() {
//         assert_eq!(
//             color(Some("XXXXXX".into()), None, "".into(), Some("".into()), 0),
//             None
//         );
//     }

//     #[test]
//     fn no_event_valid_server() {
//         assert_eq!(
//             color(None, Some("BC614E".into()), "".into(), Some("".into()), 0),
//             Some(12345678)
//         );
//     }

//     #[test]
//     fn no_event_hashtag_server() {
//         assert_eq!(
//             color(None, Some("#BC614E".into()), "".into(), Some("".into()), 0),
//             Some(12345678)
//         );
//     }

//     #[test]
//     fn no_event_invalid_server() {
//         assert_eq!(
//             color(None, Some("XXXXXX".into()), "".into(), Some("".into()), 0),
//             None
//         );
//     }

//     #[test]
//     fn no_event_no_server() {
//         assert_eq!(color(None, None, "".into(), Some("".into()), 0), None);
//     }
// }
