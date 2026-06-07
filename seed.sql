-- Fallback missions used when Ollama is unavailable
-- 20 varied missions covering different interests and activity types

INSERT INTO fallback_missions (title, description, time_limit_minutes, radius_meters, points)
VALUES
    ('Sunrise Photographer', 'Capture the sunrise from the highest point within range. Frame it creatively.', 45, 2000, 60),
    ('Urban Explorer', 'Find and photograph 3 unique examples of street art or architecture.', 30, 1500, 40),
    ('Green Thumb', 'Visit a community garden, plant shop, or green space. Learn one plant name.', 25, 1000, 30),
    ('Coffee Connoisseur', 'Try a coffee shop you have never visited. Order something outside your usual.', 20, 800, 25),
    ('Park Ranger', 'Walk through a nearby park. Identify 3 different types of trees or birds.', 30, 1500, 35),
    ('Fitness Fanatic', 'Complete 20 pushups, 30 squats, and run 1 kilometer without stopping.', 25, 500, 50),
    ('Mindfulness Master', 'Find a quiet outdoor spot. Sit still and observe for 10 minutes.', 15, 500, 20),
    ('Social Butterfly', 'Give a genuine compliment to a stranger. Smile at 5 people.', 20, 1000, 15),
    ('Food Critic', 'Try a food item you have never eaten before. Take notes on the experience.', 30, 1500, 30),
    ('Historian', 'Find a historical landmark or plaque nearby. Learn and document its story.', 25, 2000, 35),
    ('Minimalist', 'Identify 3 items at home you can donate or discard. Do it now.', 20, 100, 25),
    ('Sketch Artist', 'Sit somewhere public and sketch a building, tree, or person. 5-minute minimum.', 20, 800, 25),
    ('Music Lover', 'Listen to a full album without distractions. Write a one-sentence review.', 40, 100, 20),
    ('Writer', 'Write 100 words describing your surroundings using all five senses.', 20, 500, 25),
    ('Photographer', 'Take 5 creative photos of ordinary objects. Make them look extraordinary.', 25, 1000, 30),
    ('Nature Lover', 'Find and photograph a flower, bird, or insect. Note the species if you can.', 20, 1000, 30),
    ('Navigator', 'Walk 1 kilometer without using GPS or maps. Use only landmarks and intuition.', 25, 1500, 45),
    ('Chef', 'Cook a simple meal using only 5 ingredients. Plate it beautifully.', 35, 200, 35),
    ('Philosopher', 'Write down 3 things you are genuinely grateful for today.', 10, 100, 15),
    ('Night Owl', 'Go outside after dark and find 3 constellations or bright stars.', 25, 1000, 40)
ON CONFLICT DO NOTHING;
