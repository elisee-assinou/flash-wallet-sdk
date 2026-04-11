-- Supprime les doublons existants (garde le plus récent)
DELETE FROM wallet_config
WHERE id NOT IN (
    SELECT DISTINCT ON (momo_number, lightning_address) id
    FROM wallet_config
    ORDER BY momo_number, lightning_address, created_at DESC
);

-- Ajoute une contrainte UNIQUE sur la combinaison
ALTER TABLE wallet_config
ADD CONSTRAINT unique_momo_lightning UNIQUE (momo_number, lightning_address);
