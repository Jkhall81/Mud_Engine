-- DROP ALL TABLES (optional, for dev only)
DROP TABLE IF EXISTS player_equipment;
DROP TABLE IF EXISTS players;
DROP TABLE IF EXISTS equipment;
DROP TABLE IF EXISTS room_exits;
DROP TABLE IF EXISTS rooms;

-- =======================
-- Table: equipment
-- =======================
CREATE TABLE equipment (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    slot TEXT NOT NULL, -- e.g. 'Weapon', 'Shield', etc.
    bonus_attack INTEGER NOT NULL DEFAULT 0,
    bonus_defense INTEGER NOT NULL DEFAULT 0
);

-- =======================
-- Table: players
-- =======================
CREATE TABLE players (
    id SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    race TEXT NOT NULL,
    class TEXT NOT NULL,
    location TEXT NOT NULL,

    level INTEGER NOT NULL DEFAULT 1,
    experience INTEGER NOT NULL DEFAULT 0,

    hp INTEGER NOT NULL,
    mana INTEGER NOT NULL,
    max_hp INTEGER NOT NULL,
    max_mana INTEGER NOT NULL,

    strength INTEGER NOT NULL,
    dexterity INTEGER NOT NULL,
    constitution INTEGER NOT NULL,
    intelligence INTEGER NOT NULL,
    wisdom INTEGER NOT NULL,

    gold INTEGER NOT NULL DEFAULT 0,
    attacks_per_round SMALLINT NOT NULL DEFAULT 1
);

-- =======================
-- Table: player_equipment
-- =======================
CREATE TABLE player_equipment (
    player_id INTEGER REFERENCES players(id) ON DELETE CASCADE,
    equipment_id INTEGER REFERENCES equipment(id) ON DELETE CASCADE,
    equipped BOOLEAN NOT NULL DEFAULT FALSE,
    slot TEXT, -- required if equipped = true

    PRIMARY KEY (player_id, equipment_id)
);

-- =======================
-- Table: rooms
-- =======================
CREATE TABLE rooms (
    id TEXT PRIMARY KEY, -- e.g. "start", "forest_edge"
    description TEXT NOT NULL
);

-- =======================
-- Table: room_exits
-- =======================
CREATE TABLE room_exits (
    from_room TEXT REFERENCES rooms(id) ON DELETE CASCADE,
    direction TEXT NOT NULL, -- "north", "south", etc.
    to_room TEXT REFERENCES rooms(id) ON DELETE CASCADE,
    PRIMARY KEY (from_room, direction)
);
