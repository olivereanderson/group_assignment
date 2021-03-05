use std::collections::HashSet;
use std::hash::Hash;
use std::vec::Vec;

mod groups_of_subjects {
    #[derive(Debug)]
    struct Subject {
        id: u64,
        preferences: Vec<u64>,
    }
    impl Subject {
        fn dissatisfaction(&self, group_id: &u64) -> i32 {
            let dissatisfaction = self.preferences.iter().position(|x| x == group_id);
            dissatisfaction.unwrap_or(self.preferences.len()) as i32
        }
        fn new(id: u64, preferences: Vec<u64>) -> Subject {
            Subject { id, preferences }
        }
    }
    #[derive(Debug)]
    struct Group {
        id: u64,
        subjects: Vec<Subject>,
        capacity: i32,
        highest_dissatisfaction: i32,
    }

    impl Group {
        fn overfull(&self) -> bool {
            (self.subjects.len() as i32) > self.capacity
        }

        fn new(id: u64, mut subjects: Vec<Subject>, capacity: i32) -> Group {
            let mut highest_dissatisfaction = 0;
            if subjects.len() > 0 {
                subjects.sort_by(|a, b| a.dissatisfaction(&id).cmp(&b.dissatisfaction(&id)));
                highest_dissatisfaction = subjects.last().unwrap().dissatisfaction(&id);
            }
            Group {
                id,
                subjects,
                capacity,
                highest_dissatisfaction,
            }
        }

        fn highest_dissatisfaction(&self) -> i32 {
            self.highest_dissatisfaction
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn group_sorts_subjects_upon_creation() {
            let first_subject_id = 1 as u64;
            let second_subject_id = 2 as u64;
            let first_group_id = 101 as u64;
            let second_group_id = 102 as u64;
            let first_subject =
                Subject::new(first_subject_id, vec![second_group_id, first_group_id]);
            let second_subject =
                Subject::new(second_subject_id, vec![first_group_id, second_group_id]);
            let first_group = Group::new(first_group_id, vec![first_subject, second_subject], 1);
            assert_eq!(1, first_group.highest_dissatisfaction());
        }
    }

    mod proposals {
        use super::Group;
        use super::Subject;
        use offers::MembershipOffer;
        use offers::TransferralOffer;

        impl Group {
            fn id(&self) -> u64 {
                self.id 
            }

            fn handle_membership_proposal(&self, subject: &Subject) -> Option<MembershipOffer> {
                let dissatisfaction_rating = subject.dissatisfaction(&self.id);
                if (self.subjects.len() as i32) >= self.capacity {
                    let dissatisfaction_improvement =
                        dissatisfaction_rating - self.highest_dissatisfaction;
                    if dissatisfaction_improvement >= 0 {
                        None
                    } else {
                        Some(MembershipOffer::new(
                            dissatisfaction_rating,
                            Some((-dissatisfaction_improvement) as u32),
                        ))
                    }
                } else {
                    Some(MembershipOffer::new(dissatisfaction_rating, None))
                }
            }

            fn propose_transferral(&self, other: &Self) -> Option<TransferralOffer> {
                if let Some((subject_lookup_key, membership_offer)) = self.subjects.iter().enumerate().filter_map(|(key,subject)| {
                    match other.handle_membership_proposal(&subject) {
                        Some(membership_offer) => Some((key,membership_offer)),
                        None => None, 
                    }
                }).min_by(|(key1,offer1),(key2,offer2)| offer1.cmp(&offer2)) {
                    Some(TransferralOffer::new(subject_lookup_key,other.id(),membership_offer))
                } else {
                    None
                }
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

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
                Subject::new(first_subject_id, vec![second_group_id, first_group_id]);
            let second_subject =
                Subject::new(second_subject_id, vec![first_group_id, second_group_id]);
            let third_subject = Subject::new(third_subject_id, vec![second_group_id,first_group_id]);
            // Groups 
            let first_group = Group::new(first_group_id, vec![first_subject, second_subject], 3);
            let second_group = Group::new(second_group_id, vec![third_subject],1);
            let offer = first_group.propose_transferral(&second_group);
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
                Subject::new(first_subject_id, vec![second_group_id, first_group_id]);
            let second_subject =
                Subject::new(second_subject_id, vec![first_group_id, second_group_id]);
            let third_subject = Subject::new(third_subject_id, vec![second_group_id,first_group_id]);
            let fourth_subject = Subject::new(fourth_subject_id, vec![first_group_id,second_group_id]);
            // Groups 
            let first_group = Group::new(first_group_id, vec![first_subject, second_subject], 2);
            let second_group = Group::new(second_group_id, vec![third_subject, fourth_subject],2);
            let offer = second_group.propose_transferral(&first_group);
            let expected_offer = TransferralOffer::new(1,101,MembershipOffer::new(0,Some(1)));
            assert_eq!(offer.unwrap(),expected_offer);
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
                Subject::new(first_subject_id, vec![second_group_id, first_group_id]);
            let second_subject =
                Subject::new(second_subject_id, vec![first_group_id, second_group_id]);
            let third_subject = Subject::new(third_subject_id, vec![second_group_id,first_group_id]);
            let fourth_subject = Subject::new(fourth_subject_id, vec![second_group_id,first_group_id]);
            // Groups 
            let first_group = Group::new(first_group_id, vec![first_subject, second_subject], 3);
            let second_group = Group::new(second_group_id, vec![third_subject, fourth_subject],1);
            let offer = second_group.propose_transferral(&first_group);
            let expected_offer = TransferralOffer::new(1,101,MembershipOffer::new(1,None));
            assert_eq!(offer.unwrap(),expected_offer);
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
                Subject::new(first_subject_id, vec![second_group_id, first_group_id]);
            let second_subject =
                Subject::new(second_subject_id, vec![first_group_id, second_group_id]);
            let third_subject = Subject::new(third_subject_id, vec![first_group_id, second_group_id]);
            // Group(s)
            let first_group = Group::new(first_group_id, vec![first_subject, second_subject], 3);
            let actual_offer = first_group.handle_membership_proposal(&third_subject).unwrap(); 
            let offer = MembershipOffer::new(0,None);
            assert_eq!(offer,actual_offer);
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
                    Subject::new(first_subject_id, vec![second_group_id, first_group_id]);
                let second_subject =
                    Subject::new(second_subject_id, vec![first_group_id, second_group_id]);
                let third_subject = Subject::new(third_subject_id, vec![second_group_id, first_group_id]);
                // Group(s)
                let first_group = Group::new(first_group_id, vec![first_subject], 1);
                // The second subject wants to be in the first group more than the first so an offer is given
                let actual_offer = first_group.handle_membership_proposal(&second_subject).unwrap(); 
                let offer = MembershipOffer::new(0,Some(1));
                assert_eq!(offer,actual_offer);
                // The third subject does not want to be in the first group more than the first thus no offer is given
                let no_offer = first_group.handle_membership_proposal(&third_subject).is_none();
                assert!(no_offer);
            }

        }
        pub mod offers {
            use std::cmp::Eq;
            use std::cmp::Ord;
            use std::cmp::Ordering;
            use std::cmp::PartialOrd;

            #[derive(Eq, Debug)]
            pub struct MembershipOffer {
                dissatisfaction_rating: i32, // How dissatisfied the Subject is with the proposed group
                dissatisfaction_improvement: Option<i32>, //None if no one has to leave the group upon offer acceptance
            }

            impl MembershipOffer {
                pub fn new(
                    dissatisfaction_rating: i32,
                    dissatisfaction_improvement: Option<u32>,
                ) -> MembershipOffer {
                    MembershipOffer {
                        dissatisfaction_rating,
                        dissatisfaction_improvement: match dissatisfaction_improvement {
                            Some(value) => Some(-(value as i32)),
                            None => None,
                        },
                    }
                }
            }

            // Want to be able to sort MembershipOffer lexicographically with respect to dissatisfaction_rating and misplacement_data
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
            pub struct TransferralOffer {
                subject_lookup_key: usize,
                proposed_group_id: u64,
                membership_offer: MembershipOffer,
            }

            impl TransferralOffer {
                pub fn new(
                    subject_lookup_key: usize,
                    proposed_group_id: u64,
                    membership_offer: MembershipOffer,
                ) -> TransferralOffer {
                    TransferralOffer {
                        subject_lookup_key,
                        proposed_group_id,
                        membership_offer,
                    }
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
                    let transferral_offer_no_misplacement =
                        TransferralOffer::new(1, 101, no_misplacement);
                    let transferral_offer_with_misplacement =
                        TransferralOffer::new(2, 102, with_misplacement);
                    assert!(
                        transferral_offer_no_misplacement < transferral_offer_with_misplacement
                    );
                    assert!(
                        !(transferral_offer_with_misplacement < transferral_offer_no_misplacement)
                    );
                    assert!(
                        !(transferral_offer_no_misplacement == transferral_offer_with_misplacement)
                    );
                }

                #[test]
                fn ordering_transitivity() {
                    let no_misplacement = MembershipOffer::new(2, None);
                    let with_misplacement = MembershipOffer::new(2, Some(4));
                    let with_most_misplacement = MembershipOffer::new(2, Some(7));
                    assert!(no_misplacement < with_most_misplacement);
                    assert!(with_most_misplacement < with_misplacement);
                    assert!(no_misplacement < with_misplacement);
                    // Now check the same for the ordering on transferral offers.
                    let transferral_offer_no_misplacement =
                        TransferralOffer::new(1, 101, no_misplacement);
                    let transferral_offer_with_misplacement =
                        TransferralOffer::new(2, 102, with_misplacement);
                    let transferral_offer_with_most_misplacement =
                        TransferralOffer::new(3, 103, with_most_misplacement);
                    assert!(
                        transferral_offer_no_misplacement
                            < transferral_offer_with_most_misplacement
                    );
                    assert!(
                        transferral_offer_with_most_misplacement
                            < transferral_offer_with_misplacement
                    );
                    assert!(
                        transferral_offer_no_misplacement < transferral_offer_with_misplacement
                    );
                }

                #[test]
                fn ordering_equality() {
                    let no_misplacement = MembershipOffer::new(2, None);
                    let other_no_misplacement = MembershipOffer::new(2, None);
                    let with_misplacement = MembershipOffer::new(2, Some(4));
                    let other_with_misplacement = MembershipOffer::new(2, Some(4));
                    assert_eq!(no_misplacement, other_no_misplacement);
                    assert_eq!(with_misplacement, other_with_misplacement);
                    // Now we make similar tests for transferral offers with varying id's.
                    let transferral_offer_no_misplacement =
                        TransferralOffer::new(1, 101, no_misplacement);
                    let other_transferral_no_misplacement =
                        TransferralOffer::new(2, 102, other_no_misplacement);
                    assert_eq!(
                        transferral_offer_no_misplacement,
                        other_transferral_no_misplacement
                    );
                }
            }
        }
    }
}
