use std::{ops::Div, primitive};
// continue on page 205

#[allow(unused)]
#[rustfmt::skip]
mod domain_model {
use std::{
  collections::LinkedList,
  future::Future};
  
struct Kilograms(f64);
struct Meters(f64);

#[derive(Clone)]
struct NonEmptyList<T:Clone> {first: T, rest: LinkedList<T>}
// for unimplemented types
#[derive(Clone)]
enum Undefined        { Undefined             }
//// Order Domain
// == Value objets ==
// Product code types

#[derive(PartialEq)] 
#[derive(Clone)]
enum WidgetCode <'a>{ WidgetCode(&'a str)}
#[derive(Clone)]
enum GizmoCode  <'a>{ GizmoCode( &'a str)}
#[derive(Clone)]
enum ProductCode<'a>{ Widget(  WidgetCode<'a>),
                      Gizmo(   GizmoCode <'a>) }
// Order Quantity types
pub enum UnitQuantity { UnitQuantity(i32) }
mod _u_qty {use super::{ UnitQuantity,
                         UnitQuantity::UnitQuantity as UQV};
  impl UnitQuantity {
    pub const fn create(qty:i32)->Result<UnitQuantity,&'static str>{
      match qty {1   =>Err("UnitQuantity can not be negative"),
                 1000=>Err("UnitQuantity can not be more than 1000"),
                 _   =>Ok(  UQV(qty)) }}
    pub const fn value(&self)->i32{match *self{UQV(qty) => qty,}}
}}
enum KilogramQuantity { KilogramQuantity(Kilograms) } // no decimal type like in F#
enum OrderQuantity    { Units(UnitQuantity    ),
                        Kilos(KilogramQuantity) }

// == Entities ==
#[derive(Clone,PartialEq)]
enum OrderId         { OrderId(i32)          } // Undefined?
type OrderLineId = Undefined;
enum CustomerId       { CustomerId(i32)       }

type CustomerInfo    = Undefined;
type ShippingAddress = Undefined;
type BillingAddress  = Undefined;
type Price           = Undefined;
type BillingAmount   = Undefined;

#[derive(Clone)]
struct OrderLine <'a>{ id            : OrderLineId    ,
                       order_id      : OrderId        ,
                       product_code  : ProductCode<'a>,
                       order_quantity: i32            ,
                       price         : Price           }
// impl   OrderLine     { fn key(&self)->(OrderId,ProductId) {
//                           (self.order_id.clone(), self.product_id.clone()) }}
enum Order<'a> { Unvalidated(    UnvalidatedOrder<'a>),
                 ValidatedOrder( ValidatedOrder<  'a>),
                 Priced(         PricedOrder<     'a>) }
// struct Order<'a> { id               : OrderId                    ,
//                    customer_id      : CustomerId                 , 
//                    shipping_address : ShippingAddress            ,
//                    billing_address  : BillingAddress             ,
//                    order_lines      : NonEmptyList<OrderLine<'a>>,
//                    amount_to_bill   : BillingAmount               }

#[derive(Clone,PartialEq)]
enum   ProductId     { ProductId(i32) }

// type Address = Undefined;
type UnvalidatedAddress = Undefined;
struct ValidatedAddress(Undefined);
type AddressValidationService = fn(UnvalidatedAddress)->Option<ValidatedAddress>;
type UnvalidatedCustomerInfo = Undefined;

struct UnvalidatedOrder <'a>{ order_id         : &'a str,
                              customer_info    : UnvalidatedCustomerInfo,
                              shipping_address : UnvalidatedAddress
                              // ...
                            }

type ValidatedOrderLine = Undefined;
struct ValidatedOrder   <'a>{ order_id         : OrderId                       ,
                              customer_info    : Undefined                     ,
                              shipping_address : Address<'a>                   ,// ValidatedAddress
                              billing_address  : Address<'a>                   ,
                              order_lines      : LinkedList<ValidatedOrderLine> }
struct ValidationError<'a> { field_name        : &'a str,
                             error_description : &'a str }
type ValidationResponse<T> = fn(Result<T, LinkedList<ValidationError>>)
                                -> dyn Future<Output = T>;
type ValidateOrder         = fn(UnvalidatedOrder)
                                -> ValidationResponse<ValidatedOrder>;

type PricedOrderLine = Undefined;
struct PricedOrder<'a> { order_id         : OrderId                    ,
                         customer_info    : CustomerInfo               ,
                         shipping_address : Address<'a>                ,
                         billing_address  : Address<'a>                ,
                         orderlines       : LinkedList<PricedOrderLine>,
                         amount_to_bill   : BillingAmount               }


type AcknowledgmentSent  = Undefined;
type OrderPlaced         = Undefined;
type BillableOrderPlaced = Undefined;

struct PlacedOrderEvents { acknowledgment_sent   : AcknowledgmentSent,
                           order_placed          : OrderPlaced,
                           billable_order_placed : BillableOrderPlaced }
#[non_exhaustive]
enum PlaceOrderError <'a>{ ValidationError(LinkedList<ValidationError<'a>>)}
// type PlaceOrder          = fn(UnvalidatedOrder)->Result<PlacedOrderEvents,PlaceOrderError>; // "Place Order" process
type DateTime = Undefined;
struct Command<Data> { data       : Data    ,
                        timestamp : DateTime,
                        user_id   : String  }
type PlaceOrder<'a> = Command<UnvalidatedOrder<'a>>;
type RequestChangeOrder = Undefined;
type RequestCancelOrder = Undefined;
type ChangeOrder    = Command<RequestChangeOrder>;
type CancelOrder    = Command<RequestCancelOrder>;
enum OrderTakingCommand<'a> { Place( PlaceOrder<'a>),
                              Change(ChangeOrder)   ,
                              Cancel(CancelOrder)    }

type QuoteForm             = Undefined;
enum CategorizedMail       { Quote(QuoteForm),
                             Order(QuoteForm) }
type EnvelopeContents      = Undefined;
type CategorizeInboundMail = fn(EnvelopeContents)->CategorizedMail;

type OrderForm       = Undefined;
type ProductCatalog  = Undefined;
// type PricedOrder     = Undefined;
type CalculatePrices<'a> = fn(OrderForm,ProductCatalog)->PricedOrder<'a>;

struct CalculatePriceInput { order_form      : OrderForm,
                             product_catalog : ProductCatalog }
type CalculatePrices2<'a>      = fn(CalculatePriceInput)->PricedOrder<'a>;

enum PhoneNumber <'a>   { PhoneNumber(&'a str)  }
enum EmailAddress<'a>   { EmailAddress(&'a str) }
type VerifiedEmailAddress = Undefined;
enum CustomerEmail<'a> { Unverified(EmailAddress<'a>), Verified(VerifiedEmailAddress)}
type SendPasswordResetEmail = fn(VerifiedEmailAddress)->Undefined;
enum ContactId          { ContactId(i32)       }
type EmailContactInfo   = Undefined;
type PostalContactInfo  = Undefined;
struct BothContactMeathods { email   : EmailContactInfo ,
                             address : PostalContactInfo }
enum ContactInfo { EmailOnly(   EmailContactInfo   ),
                   AddrOnly(    PostalContactInfo  ),
                   EmailAndAddress(BothContactMeathods) }
struct Contact <'a>{ name         : Name<'a>,
                     contact_info : ContactInfo}
// OLD
// struct Contact   <'a,'b>{ contact_id   : ContactId       ,
//                           phone_number : PhoneNumber <'a>,
//                           email_address: EmailAddress<'b> }
// // I should not use this impl going forward in the book
// impl PartialEq for Contact<'_,'_> { 
//   fn eq (&self,r_a: &Self)->bool { 
//     use ContactId::ContactId as C;
//     match (&self.contact_id,&r_a.contact_id) { (C(l),C(r)) => *l==*r,}
// }}


type   InvoiceId     = Undefined;
struct UnpaidInvoice { invoice_id : InvoiceId }
struct PaidInvoice   { invoice_id : InvoiceId }
enum   Invoice       { Unpaid(UnpaidInvoice),
                       Paid(  PaidInvoice)    }

enum PersonID { PersonID(i32)}
type Name<'a> = &'a str;
struct Person<'a> { person_id: PersonID, name: Name<'a> }
const INITIAL_PERSON: Person = Person { person_id: PersonID::PersonID(42), name: "Joseph" };
const UPDATED_PERSON: Person = Person {                                    name: "Joe", .. INITIAL_PERSON };
type UpdateName<'a> = fn(Person<'a>,Name<'a>)->Person<'a>;

type MoneyTransferId = Undefined;
type AccountId       = Undefined;
type Money           = Undefined;
struct MoneyTransfer {id:           MoneyTransferId,
                      to_account:   AccountId      ,
                      from_account: AccountId      ,
                      amount:       Money           }


pub const PRINT_EQ_WIDGET: fn() = || {
  use {WidgetCode::WidgetCode as W};
  let s = String::from("W1234");
  let widget_code1 = W(s.as_str());
  let widget_code2 = W("W1234");
  println!("{}", {widget_code1==widget_code2});
};


#[derive(PartialEq)]
struct PersonalName<'a,'b> {first_name: &'a str, last_name: &'b str}
pub const PRINT_EQ_NAME: fn() = || {
  use {String as S, PersonalName as P};
  let (f,l)=(S::from("Alex"), S::from("Adams"));
  let name1 = P {
    first_name: f.as_str(),
    last_name : l.as_str() };
  let name2 = P { 
    first_name: "Alex"    ,
    last_name : "Adams"    };
  println!("{}",{name1==name2})
};

#[derive(PartialEq)]
struct Address<'a> {street_address: &'a str, city: &'a str, zip: &'a str}
pub const PRINT_EQ_ADDRESS: fn() = || { 
  use {String as S, Address as A};
  let (s,c,z) = (S::from("123 Main St"),S::from("New York"),S::from("90001"));
  let address1 = A{
    street_address: s.as_str()   ,
    city          : c.as_str()   ,
    zip           : z.as_str()    };
  let address2 = A{
    street_address: "123 Main St",
    city          : "New York"   ,
    zip           : "90001"       };
  println!("{}",{address1==address2})
};

// OLD
// pub const PRINT_EQ_CONTACT_ID: fn() = || {
//   use {PhoneNumber::PhoneNumber as P, EmailAddress::EmailAddress as E, String as S};
//   const C_ID: ContactId = ContactId::ContactId(1);
//   let (p,e) = (S::from("123-456-7890"),S::from("bob@example.com"));
//   let contact1 = Contact {
//     contact_id   : C_ID,
//     phone_number : P( p.as_str() ),
//     email_address: E( e.as_str() ) };
//   let contact2 = Contact { 
//     contact_id   : C_ID,
//     phone_number : P("123-456-7890"      ),
//     email_address: E("robert@example.com") };
//   println!("{}",{contact1==contact2});
// };

// pub const PRINT_EQ_ORDER_LINE_KEY: fn() = || {
//   use {OrderId::OrderId as O, ProductId::ProductId as P};
//   let line1 = OrderLine {order_id: O(3), product_id: P(5), qty: 2};
//   let line2 = OrderLine {order_id: O(3), product_id: P(5), qty: 2};
//   println!("{}",{line1.key()==line2.key()});
// };

pub const fn unit_qty_result()->Result<UnitQuantity,&'static str> {
  UnitQuantity::create(1)}

fn find_order_line<'a>(order_line   :NonEmptyList<OrderLine<'a>>, 
                       order_line_id:OrderLineId                 )
  ->OrderLine<'a>{todo!()}
fn replace_order_line<'a>(order_lines   : NonEmptyList<OrderLine<'a>>,
                          orderline_id  : OrderLineId               ,
                          new_order_line: OrderLine<'a>              )
  ->NonEmptyList<OrderLine<'a>>{todo!()}
// fn change_order_line_price(order         : Order       ,
//                            order_line_id : OrderLineId , 
//                            new_price     : Price        )->Order
// {
// let order_line= find_order_line(order.order_lines.clone(), order_line_id.clone());

// let new_order_line = OrderLine {price:new_price, .. order_line};
// let new_order_lines = replace_order_line(
//   order.order_lines.clone(), 
//   order_line_id.clone(), 
//   new_order_line);
// //let new_amount_to_bill = new_order_lines.iter(). // sumBy(|line|line.price)

// Order {order_lines   :new_order_lines    , 
//    // amount_to_bill: new_amount_to_bill,
//       .. order                            }
// }
}

fn main() {
  println!("Hello, world!");
  use domain_model::*;
  PRINT_EQ_WIDGET();
  PRINT_EQ_NAME();
  PRINT_EQ_ADDRESS();
  // PRINT_EQ_CONTACT_ID();
  // PRINT_EQ_ORDER_LINE_KEY();
  match unit_qty_result() { 
    Err(msg)=>println!("Failure, Message is {}", msg),
    Ok(u_qty)=>println!("innerValue is {}", u_qty.value())};

}
