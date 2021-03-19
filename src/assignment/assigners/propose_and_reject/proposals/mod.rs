pub(super) use crate::groups::Group;
use std::collections::HashMap;
pub(super) mod offers;
pub(super) use crate::assignment::assigners::GroupRegistry;
pub(super) use crate::assignment::assigners::GrowingGroupRegistry;
pub(super) use crate::assignment::assigners::SimpleGroupRegistry;
use crate::{assignment::errors::CapacityError, subjects::Subject};
use offers::MembershipOffer;
use offers::TransferralOffer;

// Decorator pattern
// The proposal handling group registry extends the simple group registry with various methods and always
// caches how dissatisfied its least happy member is with this group.
// To be able to easily access the most dissatisfied member we always keep a reference to this subject
// at the end of the subjects vector (in the underlying simple group registry)
#[derive(Debug)]
pub(super) struct ProposalHandlingGroupRegistry<'a, S, G>
where
    S: Subject,
    G: Group,
{
    delegate: SimpleGroupRegistry<'a, S, G>,
    highest_dissatisfaction: i32,
}

impl<'a, S: Subject, G: Group> GrowingGroupRegistry<'a, S>
    for ProposalHandlingGroupRegistry<'a, S, G>
{
    fn register_subject(&mut self, subject: &'a S) -> Result<(), CapacityError> {
        if self.full() {
            Err(CapacityError {})
        } else {
            // We will add the subject and make sure that we keep the members sorted by dissatisfaction
            let id = self.id();
            let new_member_dissatisfaction_rating = subject.dissatisfaction(&id);

            if let Some(position) = self
                .delegate
                .subjects
                .iter()
                .rposition(|x| x.dissatisfaction(&id) <= new_member_dissatisfaction_rating)
            {
                self.delegate.subjects.insert(position, subject);
            } else {
                print!(
                    "group : {:?}, has {:?} members",
                    self.id(),
                    self.delegate.subjects.len()
                );
                self.delegate.subjects.insert(0, subject);
            }
            self.highest_dissatisfaction = self
                .delegate
                .subjects
                .last()
                .map_or(0, |x| x.dissatisfaction(&id));
            Ok(())
        }
    }
}

impl<'a, S: Subject, G: Group> Group for ProposalHandlingGroupRegistry<'a, S, G> {
    fn id(&self) -> u64 {
        self.delegate.id()
    }

    fn capacity(&self) -> i32 {
        self.delegate.capacity()
    }
}

impl<'a, S: Subject, G: Group> GroupRegistry for ProposalHandlingGroupRegistry<'a, S, G> {
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

impl<'a, S: Subject, G: Group> ProposalHandlingGroupRegistry<'a, S, G> {
    pub(super) fn new_without_dissatisfaction(group: &'a G, subjects: Vec<&'a S>) -> Self {
        let delegate = SimpleGroupRegistry::new(group, subjects);
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
            Some(TransferralOffer::new(lookup_key, membership_offer))
        } else {
            None
        }
    }

    // removes the least happy member from the group registry and adds another member.
    // The removed member is returned.
    fn replace_least_happy_member(&mut self, subject: &'a S) -> &'a S {
        let least_happy_member = self.delegate.subjects.pop().unwrap(); // The last entry is always least happy with this group
        self.register_subject(subject).unwrap();
        least_happy_member
    }

    // Removes a member from this group registry and adds it to another under the conditions of a transferral offer.
    // If the other group registry is at full capacity its least happy member will be removed
    pub(super) fn transfer(&mut self, other: &mut Self, offer: TransferralOffer) -> Option<&'a S> {
        let subject_to_be_transferred = self.delegate.subjects.remove(offer.subject_lookup_key);
        let mut replaced_subject = None;
        if offer.replace_least_happy_member_upon_transferral() {
            replaced_subject = Some(other.replace_least_happy_member(subject_to_be_transferred));
        } else {
            other.register_subject(subject_to_be_transferred).unwrap();
        }
        replaced_subject
    }
    // Adds a member without taking capacity limitations into consideration
    // This method is typically used to return members to the group of their first choice after being replaced by
    // some other member in their previously assigned group.
    pub(super) fn force_register_subject(&mut self, subject: &'a S) -> () {
        let id = self.id();
        if let Some(position) = self
            .delegate
            .subjects
            .iter()
            .position(|x| subject.dissatisfaction(&id) <= x.dissatisfaction(&id))
        {
            self.delegate.subjects.insert(position, subject);
            // We assume that the subject will be happy to be added to this group and therefore use position over rposition.
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::groups::test_utils::TestGroup;
    use crate::subjects::test_utils::TestSubject;

    /// Practical method to have when we want to ensure a certain state when testing
    impl<'a, S: Subject, T: Group> ProposalHandlingGroupRegistry<'a, S, T> {
        fn new(group: &'a T, subjects: Vec<&'a S>) -> Self {
            let id = group.id();
            let highest_dissatisfaction = subjects
                .iter()
                .map(|x| x.dissatisfaction(&id))
                .max()
                .unwrap_or(0);
            let delegate = SimpleGroupRegistry::new(group, subjects);
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
        let first_group_registry =
            ProposalHandlingGroupRegistry::new(&first_group, vec![&first_subject, &second_subject]);
        let second_group = TestGroup::new(second_group_id, 1);
        let second_group_registry =
            ProposalHandlingGroupRegistry::new(&second_group, vec![&third_subject]);
        let offer = first_group_registry.propose_transferral(&second_group_registry);
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
        let first_group_registry =
            ProposalHandlingGroupRegistry::new(&first_group, vec![&first_subject, &second_subject]);
        let second_group = TestGroup::new(second_group_id, 2);
        let second_group_registry =
            ProposalHandlingGroupRegistry::new(&second_group, vec![&third_subject, &fourth_subject]);
        let actual_offer = second_group_registry.propose_transferral(&first_group_registry);
        let expected_offer = TransferralOffer::new(1, MembershipOffer::new(0, Some(-1)));
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
        let first_group_registry =
            ProposalHandlingGroupRegistry::new(&first_group, vec![&first_subject, &second_subject]);
        let second_group = TestGroup::new(second_group_id, 1);
        let second_group_registry =
            ProposalHandlingGroupRegistry::new(&second_group, vec![&third_subject, &fourth_subject]);
        let offer = second_group_registry.propose_transferral(&first_group_registry);
        let expected_offer = TransferralOffer::new(1, MembershipOffer::new(1, None));
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
        let first_group_registry =
            ProposalHandlingGroupRegistry::new(&first_group, vec![&first_subject, &second_subject]);
        let actual_offer = first_group_registry
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
        let first_group_registry =
            ProposalHandlingGroupRegistry::new(&first_group, vec![&first_subject]);
        // The second subject wants to be in the first group more than the first so an offer is given
        let actual_offer = first_group_registry
            .handle_membership_proposal(&second_subject)
            .unwrap();
        let expected_offer = MembershipOffer::new(0, Some(-1));
        assert_eq!(expected_offer, actual_offer);
        // The third subject does not want to be in the first group more than the first thus no offer is given
        let no_offer = first_group_registry
            .handle_membership_proposal(&third_subject)
            .is_none();
        assert!(no_offer);
    }
}
