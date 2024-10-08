use std::collections::HashMap;

use rand::{rngs::SmallRng, Rng};
use wows_box::lootbox::{LootBox, LootBoxFiller, LootBoxRewardList, LootBoxRewardType};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RandResult {
    pub amount: u32,
    pub reward_type: LootBoxRewardType,
    pub is_unique: bool,
}

impl RandResult {
    pub const fn new(amount: u32, reward_type: LootBoxRewardType, is_unique: bool) -> RandResult {
        RandResult {
            amount,
            reward_type,
            is_unique,
        }
    }
}

/// Return type: amount, type, is-unique
fn rand_slot<'a>(
    rng: &mut SmallRng,
    filler: Option<&LootBoxFiller>,
    // slot: &LootBoxSlot,
    lists: impl Iterator<Item = &'a LootBoxRewardList>,
    max_prob: f64,
    unique_rewards_list: &[u64],
) -> RandResult {
    let mut seed: f64 = rng.gen_range(0.0..max_prob);

    'list: for list in lists {
        debug_assert!(
            (list.probability - list.rewards.iter().map(|t| t.probability).sum::<f64>()).abs()
                <= 0.01
        );
        if seed < list.probability {
            if list.has_unique_rewards {
                let possible_amount = list
                    .rewards
                    .iter()
                    .filter(|t| {
                        !t.reward
                            .get_id()
                            .is_some_and(|i| unique_rewards_list.contains(&i))
                    })
                    .count();
                if possible_amount == 0 {
                    if let Some(filler) = filler {
                        return RandResult::new(
                            filler.amount,
                            filler.filler.clone(),
                            list.has_unique_rewards,
                        );
                    }
                }
                let target = rng.gen_range(0..possible_amount);
                let reward = list
                    .rewards
                    .iter()
                    .filter(|t| {
                        !t.reward
                            .get_id()
                            .is_some_and(|i| unique_rewards_list.contains(&i))
                    })
                    .nth(target)
                    .unwrap();
                return RandResult::new(
                    reward.amount,
                    reward.reward.clone(),
                    list.has_unique_rewards,
                );
            } else {
                let mut seed: f64 = rng.gen_range(0.0..list.probability);
                'no_unique: for reward in list.rewards.iter() {
                    if seed < reward.probability {
                        return RandResult::new(
                            reward.amount,
                            reward.reward.clone(),
                            list.has_unique_rewards,
                        );
                    } else {
                        seed -= reward.probability;
                        continue 'no_unique;
                    }
                }
            }
        } else {
            seed -= list.probability;
            continue 'list;
        }
    }

    unreachable!()
}

pub fn rand_single(
    rng: &mut SmallRng,
    data: &LootBox,
    unique_rewards_list: &[u64],
) -> Vec<RandResult> {
    let mut rewards: Vec<RandResult> = Vec::new();

    for slot in data.slots.iter() {
        let resp = rand_slot(
            rng,
            data.filler.as_ref(),
            slot.common.iter().chain(slot.valuable.iter()),
            1.0,
            unique_rewards_list,
        );
        rewards.push(resp);
    }

    assert_eq!(
        rewards.len(),
        data.slots.len(),
        "one slot produces one reward"
    );

    rewards
}

pub fn rand_unique(
    rng: &mut SmallRng,
    data: &LootBox,
    unique_rewards_list: &[u64],
) -> Vec<RandResult> {
    let mut rewards: Vec<RandResult> = Vec::new();

    'slot: for slot in data.slots.iter() {
        let unique_rewards: Vec<_> = slot
            .valuable
            .iter()
            .filter(|t| t.has_unique_rewards)
            .collect();

        if unique_rewards.len() == 0 {
            let resp = rand_slot(
                rng,
                data.filler.as_ref(),
                slot.common.iter().chain(slot.valuable.iter()),
                1.0,
                unique_rewards_list,
            );
            rewards.push(resp);
            continue 'slot;
        }

        let prob = unique_rewards.iter().map(|t| t.probability).sum();
        let resp = rand_slot(
            rng,
            data.filler.as_ref(),
            unique_rewards.into_iter(),
            prob,
            unique_rewards_list,
        );
        rewards.push(resp);
    }

    assert_eq!(
        rewards.len(),
        data.slots.len(),
        "one slot produces one reward"
    );

    rewards
}

/// Returns: (Type, is_guarantee): amount
pub fn rand_multi(
    rng: &mut SmallRng,
    data: &LootBox,
    times: u32,
    unique_rewards_list: &[u64],
    mut current_try: u32,
) -> HashMap<(LootBoxRewardType, bool), u32> {
    let mut map = HashMap::new();
    let mut unique_rewards_list = unique_rewards_list.to_owned();
    let guarantee = data.save_point.unwrap_or(u32::MAX);

    for _ in 0..times {
        current_try += 1;
        // dbg!(current_try);
        if current_try >= guarantee {
            let resp = rand_unique(rng, data, &unique_rewards_list);
            // dbg!(&resp);
            for reward in resp {
                let unique_id = reward.reward_type.get_id().filter(|_| reward.is_unique);
                *map.entry((reward.reward_type, true)).or_insert(0) += reward.amount;
                if let Some(unique_id) = unique_id {
                    unique_rewards_list.push(unique_id);
                }
            }
            current_try = 0;
        } else {
            let resp = rand_single(rng, data, &unique_rewards_list);
            // dbg!(&resp);
            for reward in resp {
                let unique_id = reward.reward_type.get_id().filter(|_| reward.is_unique);
                *map.entry((reward.reward_type, false)).or_insert(0) += reward.amount;
                if let Some(unique_id) = unique_id {
                    unique_rewards_list.push(unique_id);
                }
            }
        }
    }

    map
}
