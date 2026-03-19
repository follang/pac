struct ListNode {
    struct ListNode *next;
    struct ListNode *prev;
};

struct Item {
    int value;
    struct ListNode link;
};

static void list_insert_after(struct ListNode *pos, struct ListNode *node)
{
    node->next = pos->next;
    node->prev = pos;
    if (pos->next != (struct ListNode *)0) {
        pos->next->prev = node;
    }
    pos->next = node;
}

static struct Item make_item(int value)
{
    struct Item item = { value, { (struct ListNode *)0, (struct ListNode *)0 } };
    return item;
}
