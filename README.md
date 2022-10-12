# Loosely-Coupled-Pallets

Loose coupling solves the problem of coupling to a specific implementation. When loosely coupling to another pallet, you add an associated type to the pallet's configuration trait and ensure the supplied type implements the necessary behavior by specifying a trait bound.


pub trait Trait: system::Trait {
    // --snip--

    /// A type that will supply a set of members to check access control against
    type MembershipSource: AccountSet<AccountId = Self::AccountId>;
}

Many pallets throughout the ecosystem are coupled to a token through the Currency trait.

Having this associated type means that the loosely coupled variant of the check-membership pallet can be installed in any runtime that can supply it with a set of accounts to use as an access control list. The code for the AccountSet trait lives in traits/account-set/src/lib.rs directory and is quite short.


pub trait AccountSet {
    type AccountId;

    fn accounts() -> BTreeSet<Self::AccountId>;
}

We also see the loose coupling in the pallet's Cargo.toml file, where account-set is listed.


account-set = { path = '../../traits/account-set', default-features = false }

To actually get the set of members, we use the accounts method supplied by the trait.


// Get the members from the vec-set pallet
let members = T::MembershipSource::accounts();
