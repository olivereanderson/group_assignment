pub(super) use crate::groups::Group;
use std::collections::HashMap;
pub(super) mod offers;
pub(super) use crate::assignment::assigners::GroupManager;
pub(super) use crate::assignment::assigners::GrowingGroupManager;
pub(super) use crate::assignment::assigners::SimpleGroupManager;
use crate::{assignment::errors::CapacityError, subjects::Subject};
use offers::MembershipOffer;
use offers::TransferralOffer;

// Decorator pattern
// The proposal handling group manager extends the simple group manager with various methods and always
// caches how dissatisfied its least happy member is with this group.
// To be able to easily access the most dissatisfied member we always keep a reference to this subject
// at the end of the subjects vector (in the underlying simple group manager)
pub(super) struct ProposalHandlingGroupManager<'a, S, G>
where
    S: Subject,
    G: Group,
{
    delegate: SimpleGroupManager<'a, S, G>,
    highest_dissatisfaction: i32,
}

impl<'a, S: Subject, G: Group> GrowingGroupManager<'a, S>
    for ProposalHandlingGroupManager<'a, S, G>
{
    fn add_subject(&mut self, subject: &'a S) -> Result<(), CapacityError> {
        if self.full() {
            Err(CapacityError {})
        } else {
            let subject_dissatisfaction = subject.dissatisfaction(&self.id());
            if subject_dissatisfaction < self.highest_dissatisfaction {
                // we always keep a member with the highest dissatisfaction at the end of the subjects vector
                let least_happy_member = self.delegate.subjects.pop().unwrap();
                self.delegate.subjects.push(subject);
                self.delegate.subjects.push(least_happy_member);
            } else {
                self.highest_dissatisfaction = subject_dissatisfaction;
                self.delegate.subjects.push(subject);
            }
            Ok(())
        }
    }
}

impl<'a, S: Subject, G: Group> Group for ProposalHandlingGroupManager<'a, S, G> {
    fn id(&self) -> u64 {
        self.delegate.id()
    }

    fn capacity(&self) -> i32 {
        self.delegate.capacity()
    }
}

impl<'a, S: Subject, G: Group> GroupManager for ProposalHandlingGroupManager<'a, S, G> {
    fn full(&self) -> bool {
        self.delegate.full()
    }
    fn subjects_ids_to_group_id(&self) -> HashMap<u64, u64> {
        self.delegate.subjects_ids_to_group_id()
    }
    fn group_id_to_subject_ids(&self) -> HashMap<u64, Vec<u64>> {
        self.delegate.group_id_to_subject_ids()
    }
}

impl<'a, S: Subject, G: Group> ProposalHandlingGroupManager<'a, S, G> {
    pub(super) fn highest_dissatisfaction(&self) -> i32 {
        self.highest_dissatisfaction
    }

    pub(super) fn new_without_dissatisfaction(group: &'a G, subjects: Vec<&'a S>) -> Self {
        let delegate = SimpleGroupManager::new(group, subjects);
        Self {
            delegate,
            highest_dissatisfaction: 0 as i32,
        }
    }

    /// Provides a membership offer if this group is either not full or the proposing subject
    /// is more eager to be a member of this group then the currently most dissatisfied member.
    pub(super) fn handle_membership_proposal(&self, subject: &S) -> Option<MembershipOffer> {
        let dissatisfaction_rating = subject.dissatisfaction(&self.delegate.id());
        if (self.delegate.subjects.len() as i32) >= self.capacity() {
            let dissatisfaction_improvement = dissatisfaction_rating - self.highest_dissatisfaction;
            if dissatisfaction_improvement >= 0 {
                None
            } else {
                Some(MembershipOffer::new(
                    dissatisfaction_rating,
                    Some(dissatisfaction_improvement),
                ))
            }
        } else {
            Some(MembershipOffer::new(dissatisfaction_rating, None))
        }
    }

    pub(super) fn overfull(&self) -> bool {
        (self.delegate.subjects.len() as i32) > self.capacity()
    }

    /// Propose to another group to take a member from the current group.
    /// If the other group is not full a transferral offer referring to one of the subjects
    /// who minds the transferral the least is provided. In the case where the other group is full
    /// a transferal offer will only be provided if this group has a member who is more eager to be
    /// in the other group than that groups currently most dissatisfied member.
    pub(super) fn propose_transferral(&self, other: &Self) -> Option<TransferralOffer> {
        let proposed_group_id = other.id();
        if let Some((lookup_key, Some(membership_offer))) = self
            .delegate
            .subjects
            .iter()
            .enumerate()
            .min_by(|(_key1, x), (_key2, y)| {
                x.dissatisfaction(&proposed_group_id)
                    .cmp(&y.dissatisfaction(&proposed_group_id))
            })
            .map(|(key, subject)| (key, other.handle_membership_proposal(subject)))
        {
            Some(TransferralOffer::new(
                lookup_key,
                proposed_group_id,
                membership_offer,
            ))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::groups::test_utils::TestGroup;
    use crate::{
        groups,
        subjects::{self, test_utils::TestSubject},
    };
    /// Practical method to have when we want to ensure a certain state when testing
    impl<'a, S: Subject, T: Group> ProposalHandlingGroupManager<'a, S, T> {
        fn new(group: &'a T, subjects: Vec<&'a S>) -> Self {
            let id = group.id();
            let highest_dissatisfaction = subjects
                .iter()
                .map(|x| x.dissatisfaction(&id))
                .max()
                .unwrap_or(0);
            let delegate = SimpleGroupManager::new(group, subjects);
            Self {
                delegate,
                highest_dissatisfaction,
            }
        }
    }

    #[test]
    fn propose_transferral_none() {
        // Subject id's:
        let first_subject_id = 1 as u64;
        let second_subject_id = 2 as u64;
        let third_subject_id = 3 as u64;
        // Group id's
        let first_group_id = 101 as u64;
        let second_group_id = 102 as u64;
        // Subjects
        let first_subject =
            TestSubject::new(first_subject_id, vec![second_group_id, first_group_id]);
        let second_subject =
            TestSubject::new(second_subject_id, vec![first_group_id, second_group_id]);
        let third_subject =
            TestSubject::new(third_subject_id, vec![second_group_id, first_group_id]);
        // Groups
        let first_group = TestGroup::new(first_group_id, 3);
        let first_group_proxy =
            ProposalHandlingGroupManager::new(&first_group, vec![&first_subject, &second_subject]);
        let second_group = TestGroup::new(second_group_id, 1);
        let second_group_proxy =
            ProposalHandlingGroupManager::new(&second_group, vec![&third_subject]);
        let offer = first_group_proxy.propose_transferral(&second_group_proxy);
        assert!(offer.is_none());
    }

    #[test]
    fn propose_transferral_subject_replacement() {
        // Subject id's:
        let first_subject_id = 1 as u64;
        let second_subject_id = 2 as u64;
        let third_subject_id = 3 as u64;
        let fourth_subject_id = 4 as u64;
        // Group id's
        let first_group_id = 101 as u64;
        let second_group_id = 102 as u64;
        // Subjects
        let first_subject =
            TestSubject::new(first_subject_id, vec![second_group_id, first_group_id]);
        let second_subject =
            TestSubject::new(second_subject_id, vec![first_group_id, second_group_id]);
        let third_subject =
            TestSubject::new(third_subject_id, vec![second_group_id, first_group_id]);
        let fourth_subject =
            TestSubject::new(fourth_subject_id, vec![first_group_id, second_group_id]);
        // Groups
        let first_group = TestGroup::new(first_group_id, 2);
        let first_group_proxy =
            ProposalHandlingGroupManager::new(&first_group, vec![&first_subject, &second_subject]);
        let second_group = TestGroup::new(second_group_id, 2);
        let second_group_proxy =
            ProposalHandlingGroupManager::new(&second_group, vec![&third_subject, &fourth_subject]);
        let actual_offer = second_group_proxy.propose_transferral(&first_group_proxy);
        let expected_offer = TransferralOffer::new(1, 101, MembershipOffer::new(0, Some(-1)));
        assert_eq!(actual_offer.unwrap(), expected_offer);
    }

    #[test]
    fn propose_transferral_enough_space() {
        // Subject id's:
        let first_subject_id = 1 as u64;
        let second_subject_id = 2 as u64;
        let third_subject_id = 3 as u64;
        let fourth_subject_id = 4 as u64;
        // Group id's
        let first_group_id = 101 as u64;
        let second_group_id = 102 as u64;
        // Subjects
        let first_subject =
            TestSubject::new(first_subject_id, vec![second_group_id, first_group_id]);
        let second_subject =
            TestSubject::new(second_subject_id, vec![first_group_id, second_group_id]);
        let third_subject =
            TestSubject::new(third_subject_id, vec![second_group_id, first_group_id]);
        let fourth_subject =
            TestSubject::new(fourth_subject_id, vec![second_group_id, first_group_id]);
        // Groups
        let first_group = TestGroup::new(first_group_id, 3);
        let first_group_manager =
            ProposalHandlingGroupManager::new(&first_group, vec![&first_subject, &second_subject]);
        let second_group = TestGroup::new(second_group_id, 1);
        let second_group_manager =
            ProposalHandlingGroupManager::new(&second_group, vec![&third_subject, &fourth_subject]);
        let offer = second_group_manager.propose_transferral(&first_group_manager);
        let expected_offer = TransferralOffer::new(1, 101, MembershipOffer::new(1, None));
        assert_eq!(offer.unwrap(), expected_offer);
    }

    #[test]
    fn handle_membership_proposal_group_not_full() {
        // Subject id's:
        let first_subject_id = 1 as u64;
        let second_subject_id = 2 as u64;
        let third_subject_id = 3 as u64;
        // Group id's
        let first_group_id = 101 as u64;
        let second_group_id = 102 as u64;
        // Subjects
        let first_subject =
            TestSubject::new(first_subject_id, vec![second_group_id, first_group_id]);
        let second_subject =
            TestSubject::new(second_subject_id, vec![first_group_id, second_group_id]);
        let third_subject =
            TestSubject::new(third_subject_id, vec![first_group_id, second_group_id]);
        // Group(s)
        let first_group = TestGroup::new(first_group_id, 3);
        let first_group_manager =
            ProposalHandlingGroupManager::new(&first_group, vec![&first_subject, &second_subject]);
        let actual_offer = first_group_manager
            .handle_membership_proposal(&third_subject)
            .unwrap();
        let offer = MembershipOffer::new(0, None);
        assert_eq!(offer, actual_offer);
    }

    #[test]
    fn handle_membership_proposal_group_full() {
        // Subject id's:
        let first_subject_id = 1 as u64;
        let second_subject_id = 2 as u64;
        let third_subject_id = 3 as u64;
        // Group id's
        let first_group_id = 101 as u64;
        let second_group_id = 102 as u64;
        // Subjects
        let first_subject =
            TestSubject::new(first_subject_id, vec![second_group_id, first_group_id]);
        let second_subject =
            TestSubject::new(second_subject_id, vec![first_group_id, second_group_id]);
        let third_subject =
            TestSubject::new(third_subject_id, vec![second_group_id, first_group_id]);
        // Group(s)
        let first_group = TestGroup::new(first_group_id, 1);
        let first_group_proxy =
            ProposalHandlingGroupManager::new(&first_group, vec![&first_subject]);
        // The second subject wants to be in the first group more than the first so an offer is given
        let actual_offer = first_group_proxy
            .handle_membership_proposal(&second_subject)
            .unwrap();
        let expected_offer = MembershipOffer::new(0, Some(-1));
        assert_eq!(expected_offer, actual_offer);
        // The third subject does not want to be in the first group more than the first thus no offer is given
        let no_offer = first_group_proxy
            .handle_membership_proposal(&third_subject)
            .is_none();
        assert!(no_offer);
    }
}
