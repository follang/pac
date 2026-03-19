typedef struct RingBuffer {
    unsigned head;
    unsigned tail;
    unsigned capacity;
    int values[16];
} RingBuffer;

static unsigned ring_count(const RingBuffer *ring)
{
    return (ring->tail + ring->capacity - ring->head) % ring->capacity;
}

static RingBuffer ring_make(unsigned capacity)
{
    RingBuffer ring = { 0u, 0u, capacity, { 0 } };
    return ring;
}

static void ring_push(RingBuffer *ring, int value)
{
    ring->values[ring->tail] = value;
    ring->tail = (ring->tail + 1u) % ring->capacity;
}
