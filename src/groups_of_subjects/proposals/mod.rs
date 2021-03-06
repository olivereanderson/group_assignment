use super::Group;
use super::Subject;
use offers::MembershipOffer;
use offers::TransferralOffer;

impl Group {
    /// Provides a membership offer if this group is either not full or the proposing subject
    /// is more eager to be a member of this group then the currently most dissatisfied member.
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
                    Some(dissatisfaction_improvement),
                ))
            }
        } else {
            Some(MembershipOffer::new(dissatisfaction_rating, None))
        }
    }

    /// Propose to another group to take a member from the current group. 
    /// If the other group is not full a transferral offer referring to one of the subjects 
    /// who minds the transferral the least is provided. In the case where the other group is full 
    /// a transferal offer will only be provided if this group has a member who is more eager to be 
    /// in the other group than that groups currently most dissatisfied member. 
    fn propose_transferral(&self, other: &Self) -> Option<TransferralOffer> {
        if let Some((subject_lookup_key, membership_offer)) = self.subjects.iter().enumerate().filter_map(|(key,subject)| {
            match other.handle_membership_proposal(&subject) {
                Some(membership_offer) => Some((key,membership_offer)),
                None => None, 
            }
        }).min_by(|(_key1,offer1),(_key2,offer2)| offer1.cmp(&offer2)) {
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
    
