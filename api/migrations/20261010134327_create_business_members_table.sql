-- Add migration script here
CREATE TYPE role AS ENUM ('owner', 'manager', 'worker');

CREATE TABLE business_members (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    business_id UUID NOT NULL REFERENCES businesses(id) ON DELETE CASCADE,
    branch_id UUID,
    role role NOT NULL,

    /* 
    * Composite primary key ensures that a user can ONLY be a member of a business ONCE,
        regardless of their role or branch association.
    * It also allows user to be a member of multiple businesses, whether as an owner or worker.
    */
    PRIMARY KEY (user_id, business_id),
    -- composite foreign key
    FOREIGN KEY (branch_id, business_id) REFERENCES branches (id, business_id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CHECK (role = 'owner' OR branch_id IS NOT NULL)
);
