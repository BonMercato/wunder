use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderResponse {
    /// List of orders
    pub orders: Vec<Order>,
    /// Total count of orders
    pub total_count: u32,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct Order {
    /// The date where the shop decided to accept or refuse the order. Null when the order was automatically refused or accepted.
    pub acceptance_decision_date: Option<String>,
    /// Indicates if the order can be cancelled
    pub can_cancel: bool,
    /// Indicates if shops can or cannot ship orders
    pub can_shop_ship: bool,
    /// The channel of the commercial's order
    pub channel: Option<Channel>,
    /// Commercial order's identifier
    pub commercial_id: String,
    /// Order's creation date
    pub created_date: String,
    /// Shop's currency (iso format)
    pub currency_iso_code: String,
    /// Information of the customer who ordered
    pub customer: Customer,
    /// The payment's date of the order by the customer
    pub customer_debited_date: Option<String>,
    /// True if order is paid directly to seller. False otherwise.
    pub customer_directly_pays_seller: Option<bool>,
    /// Anonymized customer email usable for notifications that do not expect an answer
    pub customer_notification_email: Option<String>,
    /// Expected order delivery date
    pub delivery_date: Option<DeliveryDate>,
    /// Fulfillment information
    pub fulfillment: Fulfillment,
    /// Refunds have been requested and their cumulated amounts correspond to the sum of all remaining active order lines.
    pub fully_refunded: bool,
    /// Indicate if the customer has sent a message related to this order
    pub has_customer_message: bool,
    /// Indicate if order has incident on at least one order line
    pub has_incident: bool,
    /// Is an invoice available for this order.
    pub has_invoice: bool,
    /// Invoice information linked to the order
    pub invoice_details: Option<InvoiceDetails>,
    /// Order's last updated date
    pub last_updated_date: String,
    /// Order's additional fields
    pub order_additional_fields: Option<Vec<AdditionalField>>,
    /// Order's identifier
    pub order_id: String,
    /// Order lines
    pub order_lines: Vec<OrderLine>,
    /// Order's state
    pub order_state: String,
    /// Reason's code of the order state
    pub order_state_reason_code: Option<String>,
    /// Reason's label of the order state
    pub order_state_reason_label: Option<String>,
    /// Indicates if the price fields inclue or exclude taxes
    pub order_tax_mode: String,
    /// The payment's duration (i.e. the delay after which the order is supposed to be paid), in days.
    /// 
    /// Only applicable for PAY_ON_DUE_DATE orders, null otherwise.
    /// Note that this field has currently no impact on the order workflow, it is provided for information purposes.
    pub payment_duration: Option<u32>,
    /// The payment's type used by the customer to pay this order
    pub payment_type: String,
    /// The payment workflow followed by the order
    pub payment_workflow: Option<String>,
    /// Order's price (sum of order line's price)
    pub price: f64,
    /// Promotions' summary applied to the order
    pub promotions: Option<OrderPromotions>,
    /// Quote's id from which the order has been placed, or null if it was not from a quote
    pub quote_id: Option<String>,
    /// Order references
    pub references: Option<OrderReferences>,
    /// Shipping carrier code (associated with the Shipping company label)
    pub shipping_carrier_code: Option<String>,
    /// Shipping company
    pub shipping_company: Option<String>,
    /// Order's shipping deadline
    pub shipping_deadline: Option<String>,
    /// From where the offer is shipped
    pub shipping_from: Option<ShippingFrom>,
    /// Order's shipping price (sum of order line's shipping price)
    pub shipping_price: f64,
    /// PickUp DropOff (PUDO) point identifier
    pub shipping_pudo_id: Option<String>,
    /// Shipping tracking
    pub shipping_tracking: Option<String>,
    /// Shipping tracking url
    pub shipping_tracking_url: Option<String>,
    /// Code of shipping's type
    pub shipping_type_code: String,
    /// Label of shipping's type
    pub shipping_type_label: String,
    /// Code of shipping's zone
    pub shipping_zone_code: String,
    /// Label of shipping's zone
    pub shipping_zone_label: String,
    /// Order's total commission (sum of the order line's total commission)
    pub total_commission: f64,
    /// Total order's price (sum of the order's price and the order's shipping price).
    pub total_price: f64,
    /// Payment's transaction date
    pub transaction_date: Option<String>,
    /// Payment's transaction number
    pub transaction_number: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct Channel {
    /// Channel code
    pub code: String,
    /// Channel label
    pub label: String,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct Customer {
    /// Data related to the accounting contact in the organization
    pub accounting_contact: Option<Contact>,
    /// Customer's billing address
    pub billing_address: Address,
    /// Customer's civility
    pub civility: Option<String>,
    /// Customer's identifier
    pub customer_id: String,
    /// Data related to the delivery contact in the organization
    pub delivery_contact: Option<Contact>,
    /// Customer's first name
    pub firstname: String,
    /// Customer's last name
    pub lastname: String,
    /// Customer's locale
    pub locale: Option<String>,
    /// Data related to the organization that created the order (B2B transactions)
    pub organization: Option<Organization>,
    /// Customer's Shipping address
    pub shipping_address: Address,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct Contact {
    /// Customer contact's civility
    pub civility: Option<String>,
    /// Customer contact's identifier
    pub customer_id: String,
    /// Customer contact's firstname
    pub firstname: String,
    /// Customer contact's lastname
    pub lastname: String,
    /// Customer contact's locale
    pub locale: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct Address {
    /// Address city
    pub city: String,
    /// Civility of the person associated with the address
    pub civility: Option<String>,
    /// Company of the person associated with the address
    pub company: Option<String>,
    /// Country ISO 3166-1 code of the address
    pub country_iso_code: String,
    /// First name of the person associated with the address
    pub firstname: Option<String>,
    /// Last name of the person associated with the address
    pub lastname: String,
    /// Phone
    pub phone: Option<String>,
    /// Phone secondary
    pub phone_secondary: Option<String>,
    /// Address state
    pub state: Option<String>,
    /// First information line of the address street
    pub street_1: String,
    /// Second information line of the address street
    pub street_2: Option<String>,
    /// Address zip code
    pub zip_code: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct Organization {
    /// Address of the organization. Required for new organizations.
    pub address: Option<OrganizationAddress>,
    /// Number used to identify the customer organization as an established business in a country. 
    /// 
    /// E.g: SIRET number in France, NIF in Spain. Required for new organizations.
    pub identification_number: Option<String>,
    /// Name of the organization. Required for new organizations.
    pub name: Option<String>,
    /// Customer's organization id (from the operator's system)
    pub organization_id: String,
    /// Tax identification number of the organization.
    pub tax_identification_number: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct OrganizationAddress {
    /// Address city
    pub city: String,
    /// Country ISO 3166-1 code of the address
    /// 
    /// Used by Mirakl to format the address (back office, delivery bill, invoices...) and 
    /// to adapt the address validation to the country (see zip_code field)
    pub country_iso_code: String,
    /// Address state
    pub state: Option<String>,
    /// First information line of the address street
    pub street_1: String,
    /// Second information line of the address street
    pub street_2: Option<String>,
    /// Address zip code, mandatory except for: 
    /// 
    /// * Chile
    /// * Peru
    /// * Hong-Kong
    /// * Qatar
    /// * Bahrain
    /// * Saudi Arabia
    /// * Oman
    /// * Kuwait
    /// * Egypt
    /// * United Arab Emirates
    pub zip_code: String,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct DeliveryDate {
    /// Earliest order delivery date
    pub earliest: String,
    /// Latest order delivery date
    pub latest: String,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct Fulfillment {
    /// Fulfillment center code
    pub center: FulfillmentCenter,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct FulfillmentCenter {
    /// Center code
    pub code: String,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct InvoiceDetails {
    /// Invoice document information
    pub document_details: Option<Vec<DocumentDetail>>,
    /// Payment terms
    pub payment_terms: Option<PaymentTerms>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentDetail {
    /// Accounting document format
    pub format: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct PaymentTerms {
    /// Number of days
    pub days: u32,
    /// Payment terms type
    pub type_: String,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "type")]
pub enum AdditionalField {
    Boolean { code: String, value: String },
    Date { code: String, value: String },
    Link { code: String, value: String },
    List { code: String, value: String },
    MultipleValuesList { code: String, value: Vec<String> },
    Numeric { code: String, value: String },
    Regex { code: String, value: String },
    String { code: String, value: String },
    Textarea { code: String, value: String },
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct OrderLine {
    /// Indicate whether the order line full amount can be refunded
    pub can_refund: bool,
    /// List of the cancellations
    pub cancelations: Vec<Cancelation>,
    /// Category code of the product associated with the order line
    pub category_code: String,
    /// Category label of the product associated with the order line
    pub category_label: String,
    /// Order line's commission fee
    pub commission_fee: f64,
    /// Commission taxes
    pub commission_taxes: Option<Vec<CommissionTax>>,
    /// Order line's creation date
    pub created_date: String,
    /// Order line's debited date
    pub debited_date: Option<String>,
    /// Order line's description
    pub description: String,
    /// Order line's last updated date
    pub last_updated_date: String,
    /// Measurement information of the order line
    pub measurement: Option<Measurement>,
    /// Associated offer's id
    pub offer_id: u64,
    /// Associated offer sku (this is the sku of the offer defined by the shop)
    pub offer_sku: String,
    /// State of the offer associated with the order line
    pub offer_state_code: String,
    /// List of additional fields
    pub order_line_additional_fields: Vec<AdditionalField>,
    /// Order line's identifier
    pub order_line_id: String,
    /// Order line's index in the order
    pub order_line_index: u32,
    /// Order line's state
    pub order_line_state: String,
    /// Reason's code of the order line's state
    pub order_line_state_reason_code: Option<String>,
    /// Reason's label of the order line's state
    pub order_line_state_reason_label: Option<String>,
    /// The original unit price of the offer associated with the order line
    pub origin_unit_price: Option<f64>,
    /// Order line's price without shipping price
    pub price: f64,
    /// Price's additional information of the offer
    pub price_additional_info: Option<String>,
    /// The breakdown of the price, only available when advanced features are enabled
    pub price_amount_breakdown: Option<PriceAmountBreakdown>,
    /// Unit price for the offer associated with the order line
    pub price_unit: f64,
    /// List of all product's media associated to the order line
    pub product_medias: Vec<ProductMedia>,
    /// Sku of the product associated with the order line
    pub product_sku: String,
    /// Title of the product associated with the order line
    pub product_title: String,
    /// List of promotions
    pub promotions: Vec<Promotion>,
    /// Purchase information
    pub purchase_information: Option<PurchaseInformation>,
    /// Product's quantity for the order line
    pub quantity: u32,
    /// Product's date of receipt
    pub received_date: Option<String>,
    /// List of the refunds
    pub refunds: Vec<Refund>,
    /// Order line's shipped date
    pub shipped_date: Option<String>,
    /// From where (country or full address) the order line is shipped
    pub shipping_from: Option<ShippingFrom>,
    /// Total price of the order line's shipping price
    pub shipping_price: f64,
    /// The breakdown of the shipping price, only available when advanced features are enabled
    pub shipping_price_amount_breakdown: Option<PriceAmountBreakdown>,
    /// List of taxes applied on shipping charges
    pub shipping_taxes: Option<Vec<Tax>>,
    /// List of taxes applied on product price
    pub taxes: Option<Vec<Tax>>,
    /// Order line's total commission (sum of the commission fee and the commission vat)
    pub total_commission: f64,
    /// Order line's price with shipping price.
    pub total_price: f64,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct Cancelation {
    /// Cancellation's amount
    pub amount: Option<f64>,
    /// The breakdown of the cancellation's amount, only available when advanced features are enabled
    pub amount_breakdown: Option<PriceAmountBreakdown>,
    /// Cancellation's commission amount
    pub commission_amount: f64,
    /// Cancellation's commission taxes
    pub commission_taxes: Vec<CommissionTax>,
    /// The total commission amount of the cancellation (commission amount + commission taxes)
    pub commission_total_amount: f64,
    /// Cancellation's creation date
    pub created_date: String,
    /// Cancellation's id
    pub id: String,
    /// Cancelation's purchase information
    pub purchase_information: Option<PurchaseInformation>,
    /// The quantity of products canceled (This quantity is informative only)
    pub quantity: Option<u32>,
    /// Cancellation reason's code
    pub reason_code: String,
    /// Cancellation's shipping amount
    pub shipping_amount: Option<f64>,
    /// The breakdown of the cancellation's shipping amount, only available when advanced features are enabled
    pub shipping_amount_breakdown: Option<PriceAmountBreakdown>,
    /// The taxes on the shipping price
    pub shipping_taxes: Option<Vec<Tax>>,
    /// The taxes on the price
    pub taxes: Option<Vec<Tax>>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct PriceAmountBreakdown {
    pub parts: Vec<PriceAmountBreakdownPart>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct PriceAmountBreakdownPart {
    /// Part's amount. The sum of each part's amount is equal to the total amount.
    pub amount: f64,
    /// Should this amount be applied to the sellers commissions calculation.
    pub commissionable: Option<bool>,
    /// Should this amount be debited to the customer.
    /// If true, the amount is taken into account when generating the debit file.
    /// If false, the amount is not taken into account when generating the debit file.
    pub debitable_from_customer: Option<bool>,
    /// Should this amount be paid to the shop.
    // If true, the amount is taken into account when generating the shop payment voucher.
    // If false, the amount is not taken into account when generating the shop payment voucher.
    pub payable_to_shop: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct CommissionTax {
    /// Tax amount
    pub amount: f64,
    ///  Tax code
    pub code: String,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct PurchaseInformation {
    /// Cancelation's purchase commission fee
    pub purchase_comission_on_price: f64,
    /// Cancelation's purchase commission fee on shipping
    pub purchase_comission_on_shipping: f64,
    /// Cancelation's purchase price excluding shipping charges
    pub purchase_price: f64,
    /// Cancelation's purchase shipping charges
    pub purchase_shipping_price: f64,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct Tax {
    /// Tax amount
    pub amount: f64,
    /// The breakdown of the tax amount, only available when advanced features are enabled
    pub amount_breakdown: Option<PriceAmountBreakdown>,
    /// Tax code
    pub code: String,
    /// Purchase tax information
    pub purchase_tax: Option<PurchaseTax>,
    /// Tax rate
    pub rate: Option<f64>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct PurchaseTax {
    /// Purchase tax amount
    pub purchase_amount: f64,
    /// Purchase tax rate
    pub purchase_rate: Option<f64>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ShippingFrom {
    /// Describe the address from where the offer is shipped
    pub address: Option<ShippingFromAddress>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ShippingFromAddress {
    /// Address City
    pub city: Option<String>,
    /// Address Country ISO 3166-1 alpha-3 code (3 letters country code)
    pub country_iso_code: String,
    /// Address State
    pub state: Option<String>,
    /// First information line of the address street
    pub street_1: Option<String>,
    /// Second information line of the address street
    pub street_2: Option<String>,
    /// Address zip code
    pub zip_code: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct Refund {
    /// Refund's amount
    pub amount: Option<f64>,
    /// The breakdown of the refund's amount, only available when advanced features are enabled
    pub amount_breakdown: Option<PriceAmountBreakdown>,
    /// Refund's commission amount
    pub commission_amount: f64,
    /// Refund's commission taxes
    pub commission_taxes: Vec<CommissionTax>,
    /// The total commission amount of the refund (commission amount + commission taxes)
    pub commission_total_amount: f64,
    /// Refund's creation date
    pub created_date: String,
    /// Refund's id
    pub id: String,
    /// Refund's purchase information
    pub purchase_information: Option<PurchaseInformation>,
    /// The quantity of products canceled (This quantity is informative only)
    pub quantity: Option<u32>,
    /// Reason's code of the refund
    pub reason_code: String,
    /// Refund's shipping amount
    pub shipping_amount: Option<f64>,
    /// The breakdown of the refund's shipping amount, only available when advanced features are enabled
    pub shipping_amount_breakdown: Option<PriceAmountBreakdown>,
    /// The taxes on the shipping price
    pub shipping_taxes: Option<Vec<Tax>>,
    /// Refund's state
    pub state: String,
    /// The taxes on the price
    pub taxes: Option<Vec<Tax>>,
    /// The transaction date of the refund payment
    pub transaction_date: String,
    /// The transaction number of the refund payment
    pub transaction_number: String,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct Promotion {
    /// Whether the promotion's deduced amount has been divided among multiple order lines
    pub apportioned: bool,
    /// Configuration used to calculate the applied promotion
    pub configuration: Option<PromotionConfiguration>,
    /// Promotion's amount for this line.
    pub deduced_amount: f64,
    /// Promotion's id, defined by the shop
    pub id: String,
    /// The quantity of free items offered by the promotion for this line.
    /// Only applicable when promotion is of type FREE_ITEMS, null otherwise.
    pub offered_quantity: Option<u32>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct PromotionConfiguration {
    /// The amount off of the promotion.
    /// Only applicable when promotion is of type AMOUNT_OFF, null otherwise.
    pub amount_off: Option<f64>,
    /// The free item target quantity of the promotion.
    /// Only applicable when promotion is of type FREE_ITEMS, null otherwise.
    pub free_items_quantity: Option<u32>,
    /// Promotion's Internal description, set by the shop at creation
    pub internal_description: String,
    /// The percentage off of the promotion.
    /// Only applicable when promotion is of type PERCENTAGE_OFF, null otherwise.
    pub percentage_off: Option<f64>,
    /// Promotion's type, whether it is applied to this item (ITEM), or every items in the basket for this shop (BASKET)
    pub promotion_type: String,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ProductMedia {
    /// Media URL
    pub media_url: String,
    /// Media MIME Type (.jpg, .png ...)
    pub mime_type: String,
    /// Media TYPE (small, large...)
    pub type_: String,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct Measurement {
    /// Actual measurement on the order line
    pub actual_measurement: Option<f64>,
    /// Adjustment limit of the order line
    pub adjustment_limit: Option<f64>,
    /// Measurement unit of the order line
    pub measurement_unit: Option<String>,
    /// Total measurement ordered on the order line
    pub ordered_measurement: Option<f64>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct OrderReferences {
    /// Order reference for customer
    pub order_reference_for_customer: Option<String>,
    /// Order reference for seller
    pub order_reference_for_seller: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct OrderPromotions {
    /// The promotions applied on this order
    pub applied_promotions: Vec<Promotion>,
    /// The total amount deduced on this order thanks to the promotions
    pub total_deduced_amount: f64,
}