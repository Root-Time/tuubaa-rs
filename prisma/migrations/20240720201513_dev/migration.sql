-- CreateTable
CREATE TABLE "Config" (
    "name" TEXT NOT NULL,
    "id" INTEGER NOT NULL,

    CONSTRAINT "Config_pkey" PRIMARY KEY ("name")
);

-- CreateIndex
CREATE UNIQUE INDEX "Config_name_key" ON "Config"("name");
