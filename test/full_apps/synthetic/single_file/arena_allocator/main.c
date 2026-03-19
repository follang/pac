typedef struct ArenaBlock {
    unsigned char *data;
    unsigned int used;
    unsigned int capacity;
} ArenaBlock;

typedef struct Arena {
    ArenaBlock blocks[4];
    unsigned int block_count;
} Arena;

static void arena_init(Arena *arena) {
    unsigned int i;
    arena->block_count = 4;
    for (i = 0; i < arena->block_count; ++i) {
        arena->blocks[i].data = (unsigned char *)0;
        arena->blocks[i].used = 0;
        arena->blocks[i].capacity = 256u << i;
    }
}

static ArenaBlock *arena_current_block(Arena *arena) {
    return &arena->blocks[arena->block_count - 1];
}

static unsigned char *arena_alloc(Arena *arena, unsigned int size) {
    ArenaBlock *block = arena_current_block(arena);
    unsigned int offset = block->used;
    if (offset + size > block->capacity) {
        return (unsigned char *)0;
    }
    block->used += size;
    return block->data + offset;
}

int main(void) {
    Arena arena;
    unsigned char *buffer;
    arena_init(&arena);
    buffer = arena_alloc(&arena, 32);
    return buffer == (unsigned char *)0;
}
