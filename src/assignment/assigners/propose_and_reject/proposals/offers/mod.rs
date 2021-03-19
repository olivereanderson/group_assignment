// Helper module offering functionality assisting proposal handling registries in proposing transferral of subjects.
use std::cmp::Eq;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp::PartialOrd;

#[derive(Eq, Debug)]
/// An offer provided after a subject proposes to be a member of a given group.
pub(in crate::assignment::assigners::propose_and_reject) struct MembershipOffer {
    // How dissatisfied the Subject is with the proposed group. Needs to be recorded
    // in order for membership offers to be compared.
    dissatisfaction_rating: i32,
    //None if no one has to leave the group upon offer acceptance. Otherwise a negative value must be provided
    // corresponding to subject.dissatisfaction() - highest dissatisfaction rating amoung the groups current members
    dissatisfaction_improvement: Option<i32>,
}

impl MembershipOffer {
    pub(in crate::assignment::assigners::propose_and_reject) fn new(
        dissatisfaction_rating: i32,
        dissatisfaction_improvement: Option<i32>, // if a value is provided it must be negative
    ) -> MembershipOffer {
        MembershipOffer {
            dissatisfaction_rating,
            dissatisfaction_improvement,
        }
    }
}

// Want to be able to sort MembershipOffer lexicographically with respect to dissatisfaction_rating and dissatisfaction_improvement
impl Ord for MembershipOffer {
    fn cmp(&self, other: &Self) -> Ordering {
        let ordering = self
            .dissatisfaction_rating
            .cmp(&other.dissatisfaction_rating);
        match ordering {
            // If the dissatisfaction ratings are the same, then the one with the greatest displacement should
            // be considered the smallest of the two proposals.
            Ordering::Equal => self
                .dissatisfaction_improvement
                .cmp(&other.dissatisfaction_improvement),
            _ => ordering,
        }
    }
}

impl PartialOrd for MembershipOffer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for MembershipOffer {
    fn eq(&self, other: &Self) -> bool {
        match self.cmp(other) {
            Ordering::Equal => true,
            _ => false,
        }
    }
}

#[derive(Eq, Debug)]
pub(in crate::assignment::assigners::propose_and_reject) struct TransferralOffer {
    pub subject_lookup_key: usize,
    membership_offer: MembershipOffer,
}

impl TransferralOffer {
    pub(in crate::assignment::assigners::propose_and_reject) fn new(
        subject_lookup_key: usize,
        membership_offer: MembershipOffer,
    ) -> TransferralOffer {
        TransferralOffer {
            subject_lookup_key,
            membership_offer,
        }
    }

    pub fn replace_least_happy_member_upon_transferral(&self) -> bool {
        self.membership_offer.dissatisfaction_improvement.is_some()
    }
}

// Order TransferralOffer only by their membership_offer values
impl Ord for TransferralOffer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.membership_offer.cmp(&other.membership_offer)
    }
}

impl PartialOrd for TransferralOffer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TransferralOffer {
    fn eq(&self, other: &Self) -> bool {
        self.membership_offer.eq(&other.membership_offer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordering_asymmetry() {
        let no_misplacement = MembershipOffer::new(2, None);
        let with_misplacement = MembershipOffer::new(2, Some(4));
        assert!(no_misplacement < with_misplacement);
        assert!(!(with_misplacement < no_misplacement));
        assert!(!(no_misplacement == with_misplacement));
        // Now check the same for the ordering on transferral offers.
        let transferral_offer_no_misplacement = TransferralOffer::new(1, no_misplacement);
        let transferral_offer_with_misplacement = TransferralOffer::new(2, with_misplacement);
        assert!(transferral_offer_no_misplacement < transferral_offer_with_misplacement);
        assert!(!(transferral_offer_with_misplacement < transferral_offer_no_misplacement));
        assert!(!(transferral_offer_no_misplacement == transferral_offer_with_misplacement));
    }

    #[test]
    fn ordering_transitivity() {
        let no_misplacement = MembershipOffer::new(2, None);
        let with_misplacement = MembershipOffer::new(2, Some(-4));
        let with_most_misplacement = MembershipOffer::new(2, Some(-7));
        assert!(no_misplacement < with_most_misplacement);
        assert!(with_most_misplacement < with_misplacement);
        assert!(no_misplacement < with_misplacement);
        // Now check the same for the ordering on transferral offers.
        let transferral_offer_no_misplacement = TransferralOffer::new(1, no_misplacement);
        let transferral_offer_with_misplacement = TransferralOffer::new(2, with_misplacement);
        let transferral_offer_with_most_misplacement =
            TransferralOffer::new(3, with_most_misplacement);
        assert!(transferral_offer_no_misplacement < transferral_offer_with_most_misplacement);
        assert!(transferral_offer_with_most_misplacement < transferral_offer_with_misplacement);
        assert!(transferral_offer_no_misplacement < transferral_offer_with_misplacement);
    }

    #[test]
    fn ordering_equality() {
        let no_misplacement = MembershipOffer::new(2, None);
        let other_no_misplacement = MembershipOffer::new(2, None);
        let with_misplacement = MembershipOffer::new(2, Some(-4));
        let other_with_misplacement = MembershipOffer::new(2, Some(-4));
        assert_eq!(no_misplacement, other_no_misplacement);
        assert_eq!(with_misplacement, other_with_misplacement);
        // Now we make similar tests for transferral offers with varying id's.
        let transferral_offer_no_misplacement = TransferralOffer::new(1, no_misplacement);
        let other_transferral_no_misplacement = TransferralOffer::new(2, other_no_misplacement);
        assert_eq!(
            transferral_offer_no_misplacement,
            other_transferral_no_misplacement
        );
    }
}
