// This file models the `products.json` file used in Shopify stores.
// Most of the fields have been commented out so that they can be
// ignored in the deserialization process as they are not used by the
// program, which should save some time. This is a bit of a shame
// considering how the file structure had been completely modeled.
// Still, some of them may become useful in the future, either for this
// monitor or in other projects, so I won't remove them.

use serde::Deserialize;

#[derive(Deserialize)]
pub struct File {
    pub products: Vec<Product>,
}

#[derive(Deserialize)]
pub struct Product {
    pub id: u64,
    pub title: String,
    pub handle: String,
    /* pub body_html: String, */
    /* pub published_at: String, */
    /* pub created_at: String, */
    pub updated_at: String,
    pub vendor: String,
    /* pub product_type: String, */
    /* pub tags: Vec<String>, */
    pub variants: Vec<Variant>,
    pub images: Vec<Image>,
    /* pub options: Vec<ProductOption>, */
}

#[derive(Deserialize)]
pub struct Variant {
    pub id: u64,
    pub title: String,
    /* pub option1: String, */
    /* pub option2: Option<String>, */
    /* pub option3: Option<String>, */
    /* pub sku: String, */
    /* pub requires_shipping: bool, */
    /* pub taxable: bool, */

    // // In the `products.json` files I've checked so far,
    // // `featured_image` always has a `null` value, therefore I'm not
    // // sure what its type is. I will temporarily assume it's a string
    // // until I find out.
    // pub featured_image: Option<String>,

    // Turns out its not a string but a map...
    /* pub featured_image: Option<FeaturedImage>, */
    pub available: bool,
    pub price: String,
    /* pub grams: u32, */

    // The same applies for `compare_at_price`.
    /* pub compare_at_price: Option<String>, */
    /* pub position: u32, */
    /* pub product_id: u64, */
    /* pub created_at: String, */
    /* pub updated_at: String, */
}

/*
#[derive(Deserialize)]
pub struct FeaturedImage {
    pub id: u64,
    pub product_id: u64,
    pub position: u32,
    pub created_at: String,
    pub updated_at: String,
    pub alt: String,
    pub width: u32,
    pub height: u32,
    pub src: String,
    pub variant_ids: Vec<u64>,
}
*/

#[derive(Deserialize)]
pub struct Image {
    /* pub id: u64, */
    /* pub created_at: String, */
    /* pub position: u32, */
    /* pub updated_at: String, */
    /* pub product_id: u64, */

    // Similarly, I've only encountered `variant_ids` in the form of
    // empty vectors, and assume it contains integers as that is the
    // type used for other ID values, however I am not certain.
    /* pub variant_ids: Vec<u64>, */
    pub src: String,
    /* pub width: u32, */
    /* pub height: u32, */
}

/*
#[derive(Deserialize)]
pub struct ProductOption {
    pub name: String,
    pub position: u32,
    pub values: Vec<String>,
}
*/
