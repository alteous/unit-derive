extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(UnitAdd)]
pub fn unit_add(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    let name = &ast.ident;
    let quote = quote! {
        impl std::ops::Add for #name {
            type Output = #name;

            fn add(self, other: #name) -> #name {
                #name(self.0 + other.0)
            }
        }
        impl<'a, 'b> std::ops::Add<&'b #name> for &'a #name {
            type Output = #name;

            fn add(self, other: &'b #name) -> #name {
                #name(self.0 + other.0)
            }
        }
    };
    quote.parse().unwrap()
}

#[proc_macro_derive(UnitAddAssign)]
pub fn unit_add_assign(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    let name = &ast.ident;
    let quote = quote! {
        impl std::ops::AddAssign for #name {
            fn add_assign(&mut self, other: #name) {
                *self = #name(self.0 + other.0)
            }
        }
        impl<'b> std::ops::AddAssign<&'b #name> for #name {

            fn add_assign(&mut self, other: &'b #name) {
                *self = #name(self.0 + other.0)
            }
        }
    };
    quote.parse().unwrap()
}

#[proc_macro_derive(UnitSub)]
pub fn unit_sub(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    let name = &ast.ident;
    let quote = quote! {
        impl std::ops::Sub for #name {
            type Output = #name;

            fn sub(self, other: #name) -> #name {
                #name(self.0 - other.0)
            }
        }
        impl<'a, 'b> std::ops::Sub<&'b #name> for &'a #name {
            type Output = #name;

            fn sub(self, other: &'b #name) -> #name {
                #name(self.0 - other.0)
            }
        }
    };
    quote.parse().unwrap()
}

#[proc_macro_derive(UnitSubAssign)]
pub fn unit_sub_assign(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    let name = &ast.ident;
    let quote = quote! {
        impl std::ops::SubAssign for #name {
            fn sub_assign(&mut self, other: #name) {
                *self = #name(self.0 - other.0)
            }
        }
        impl<'b> std::ops::SubAssign<&'b #name> for #name {

            fn sub_assign(&mut self, other: &'b #name) {
                *self = #name(self.0 - other.0)
            }
        }
    };
    quote.parse().unwrap()
}

#[proc_macro_derive(UnitMul)]
pub fn unit_mul(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    let name = &ast.ident;
    let quote = quote! {
        impl std::ops::Mul for #name {
            type Output = #name;

            fn mul(self, other: #name) -> #name {
                #name(self.0 * other.0)
            }
        }
        impl<'a, 'b> std::ops::Mul<&'b #name> for &'a #name {
            type Output = #name;

            fn mul(self, other: &'b #name) -> #name {
                #name(self.0 * other.0)
            }
        }
    };
    quote.parse().unwrap()
}

#[proc_macro_derive(UnitMulAssign)]
pub fn unit_mul_assign(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    let name = &ast.ident;
    let quote = quote! {
        impl std::ops::MulAssign for #name {
            fn mul_assign(&mut self, other: #name) {
                *self = #name(self.0 * other.0)
            }
        }
        impl<'b> std::ops::MulAssign<&'b #name> for #name {

            fn mul_assign(&mut self, other: &'b #name) {
                *self = #name(self.0 * other.0)
            }
        }
    };
    quote.parse().unwrap()
}

#[proc_macro_derive(UnitDiv)]
pub fn unit_div(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    let name = &ast.ident;
    let quote = quote! {
        impl std::ops::Div for #name {
            type Output = #name;

            fn div(self, other: #name) -> #name {
                #name(self.0 / other.0)
            }
        }
        impl<'a, 'b> std::ops::Div<&'b #name> for &'a #name {
            type Output = #name;

            fn div(self, other: &'b #name) -> #name {
                #name(self.0 / other.0)
            }
        }
    };
    quote.parse().unwrap()
}

#[proc_macro_derive(UnitDivAssign)]
pub fn unit_div_assign(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    let name = &ast.ident;
    let quote = quote! {
        impl std::ops::DivAssign for #name {
            fn div_assign(&mut self, other: #name) {
                *self = #name(self.0 / other.0)
            }
        }
        impl<'b> std::ops::DivAssign<&'b #name> for #name {

            fn div_assign(&mut self, other: &'b #name) {
                *self = #name(self.0 / other.0)
            }
        }
    };
    quote.parse().unwrap()
}

#[proc_macro_derive(UnitRem)]
pub fn unit_rem(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    let name = &ast.ident;
    let quote = quote! {
        impl std::ops::Rem for #name {
            type Output = #name;

            fn rem(self, other: #name) -> #name {
                #name(self.0 % other.0)
            }
        }
        impl<'a, 'b> std::ops::Rem<&'b #name> for &'a #name {
            type Output = #name;

            fn rem(self, other: &'b #name) -> #name {
                #name(self.0 % other.0)
            }
        }
    };
    quote.parse().unwrap()
}


#[proc_macro_derive(UnitNeg)]
pub fn unit_neg(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    let name = &ast.ident;
    let quote = quote! {
        impl std::ops::Neg for #name {
            type Output = #name;

            fn neg(self) -> #name {
                #name(-self.0)
            }
        }
        impl<'a> std::ops::Neg for &'a #name {
            type Output = #name;

            fn neg(self) -> #name {
                #name(-self.0)
            }
        }
    };
    quote.parse().unwrap()
}




// This panicks, figure out why it doesn't work.
#[proc_macro_derive(UnitAll)]
pub fn unit_all(input: TokenStream) -> TokenStream {
    let add = unit_add(input);
    let sub = unit_sub(add);
    let mul = unit_mul(sub);
    let div = unit_div(mul);
    let rem = unit_rem(div);
    let neg = unit_neg(rem);
    neg
}