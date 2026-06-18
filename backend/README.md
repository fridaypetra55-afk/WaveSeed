# Backend (wavespeed)

This scaffold includes a minimal NestJS app and Prisma schema for farms and KYC records. To run locally:

```bash
cd infra
docker-compose up -d
cd ../backend
npm install
export DATABASE_URL="postgresql://waveseed:password@localhost:5432/waveseed"
npx prisma generate
npm run start:dev
```
