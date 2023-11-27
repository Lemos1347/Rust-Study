-- Ativa a extensão para suportar UUIDs
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Cria a tabela User
CREATE TABLE IF NOT EXISTS "User" (
    "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    "username" VARCHAR(50) NOT NULL,
    "password" VARCHAR(50) NOT NULL,
    "email" VARCHAR(150) UNIQUE NOT NULL,
    "created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Trigger para atualizar a coluna updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Associa o trigger à tabela User
CREATE TRIGGER update_user_updated_at
BEFORE UPDATE ON "User"
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();
