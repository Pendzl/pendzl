#[pendzl::trait_definition]
pub trait Trait1 {
    #[ink(message)]
    fn foo(&mut self) -> bool;
}

#[pendzl::wrapper]
type Trait1Ref = Trait1;

#[pendzl::trait_definition]
pub trait Trait2 {
    #[ink(message)]
    fn bar(&mut self, callee: pendzl::traits::AccountId) -> bool {
        Trait1Ref::foo(&callee)
    }
}

#[pendzl::wrapper]
type Trait1and2Ref = Trait1 + Trait2;

fn main() {}
