use std::sync::LazyLock;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::models::MissionData;

static FALLBACK_MISSIONS: LazyLock<Vec<MissionData>> = LazyLock::new(|| {
    vec![
        MissionData {
            title: "Sunrise Photographer".into(),
            description: "Capture the sunrise from the highest point within range. Frame it creatively.".into(),
            time_limit_minutes: 45,
            radius_meters: 2000,
            points: 60,
        },
        MissionData {
            title: "Urban Explorer".into(),
            description: "Find and photograph 3 unique examples of street art or architecture.".into(),
            time_limit_minutes: 30,
            radius_meters: 1500,
            points: 40,
        },
        MissionData {
            title: "Green Thumb".into(),
            description: "Visit a community garden, plant shop, or green space. Learn one plant name.".into(),
            time_limit_minutes: 25,
            radius_meters: 1000,
            points: 30,
        },
        MissionData {
            title: "Coffee Connoisseur".into(),
            description: "Try a coffee shop you have never visited. Order something outside your usual.".into(),
            time_limit_minutes: 20,
            radius_meters: 800,
            points: 25,
        },
        MissionData {
            title: "Park Ranger".into(),
            description: "Walk through a nearby park. Identify 3 different types of trees or birds.".into(),
            time_limit_minutes: 30,
            radius_meters: 1500,
            points: 35,
        },
        MissionData {
            title: "Fitness Fanatic".into(),
            description: "Complete 20 pushups, 30 squats, and run 1 kilometer without stopping.".into(),
            time_limit_minutes: 25,
            radius_meters: 500,
            points: 50,
        },
        MissionData {
            title: "Mindfulness Master".into(),
            description: "Find a quiet outdoor spot. Sit still and observe for 10 minutes.".into(),
            time_limit_minutes: 15,
            radius_meters: 500,
            points: 20,
        },
        MissionData {
            title: "Social Butterfly".into(),
            description: "Give a genuine compliment to a stranger. Smile at 5 people.".into(),
            time_limit_minutes: 20,
            radius_meters: 1000,
            points: 15,
        },
        MissionData {
            title: "Food Critic".into(),
            description: "Try a food item you have never eaten before. Take notes on the experience.".into(),
            time_limit_minutes: 30,
            radius_meters: 1500,
            points: 30,
        },
        MissionData {
            title: "Historian".into(),
            description: "Find a historical landmark or plaque nearby. Learn and document its story.".into(),
            time_limit_minutes: 25,
            radius_meters: 2000,
            points: 35,
        },
        MissionData {
            title: "Minimalist".into(),
            description: "Identify 3 items at home you can donate or discard. Do it now.".into(),
            time_limit_minutes: 20,
            radius_meters: 100,
            points: 25,
        },
        MissionData {
            title: "Sketch Artist".into(),
            description: "Sit somewhere public and sketch a building, tree, or person. 5-minute minimum.".into(),
            time_limit_minutes: 20,
            radius_meters: 800,
            points: 25,
        },
        MissionData {
            title: "Music Lover".into(),
            description: "Listen to a full album without distractions. Write a one-sentence review.".into(),
            time_limit_minutes: 40,
            radius_meters: 100,
            points: 20,
        },
        MissionData {
            title: "Writer".into(),
            description: "Write 100 words describing your surroundings using all five senses.".into(),
            time_limit_minutes: 20,
            radius_meters: 500,
            points: 25,
        },
        MissionData {
            title: "Photographer".into(),
            description: "Take 5 creative photos of ordinary objects. Make them look extraordinary.".into(),
            time_limit_minutes: 25,
            radius_meters: 1000,
            points: 30,
        },
        MissionData {
            title: "Nature Lover".into(),
            description: "Find and photograph a flower, bird, or insect. Note the species if you can.".into(),
            time_limit_minutes: 20,
            radius_meters: 1000,
            points: 30,
        },
        MissionData {
            title: "Navigator".into(),
            description: "Walk 1 kilometer without using GPS or maps. Use only landmarks and intuition.".into(),
            time_limit_minutes: 25,
            radius_meters: 1500,
            points: 45,
        },
        MissionData {
            title: "Chef".into(),
            description: "Cook a simple meal using only 5 ingredients. Plate it beautifully.".into(),
            time_limit_minutes: 35,
            radius_meters: 200,
            points: 35,
        },
        MissionData {
            title: "Philosopher".into(),
            description: "Write down 3 things you are genuinely grateful for today.".into(),
            time_limit_minutes: 10,
            radius_meters: 100,
            points: 15,
        },
        MissionData {
            title: "Night Owl".into(),
            description: "Go outside after dark and find 3 constellations or bright stars.".into(),
            time_limit_minutes: 25,
            radius_meters: 1000,
            points: 40,
        },
    ]
});

pub fn get_fallback_missions() -> Vec<MissionData> {
    let mut rng = thread_rng();
    FALLBACK_MISSIONS
        .choose_multiple(&mut rng, 3)
        .cloned()
        .collect()
}
