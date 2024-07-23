-- CreateTable
CREATE TABLE "Voice" (
    "voice_id" INTEGER NOT NULL,
    "owner_id" INTEGER NOT NULL,
    "private" BOOLEAN NOT NULL DEFAULT false,

    CONSTRAINT "Voice_pkey" PRIMARY KEY ("voice_id")
);

-- CreateTable
CREATE TABLE "VoiceAccess" (
    "voice_id" INTEGER NOT NULL,
    "user_id" INTEGER NOT NULL,

    CONSTRAINT "VoiceAccess_pkey" PRIMARY KEY ("voice_id","user_id")
);

-- CreateTable
CREATE TABLE "UserConfig" (
    "userId" INTEGER NOT NULL,
    "ping" BOOLEAN NOT NULL,
    "prefix" TEXT NOT NULL,
    "suffix" TEXT NOT NULL,
    "limit" INTEGER NOT NULL,

    CONSTRAINT "UserConfig_pkey" PRIMARY KEY ("userId")
);

-- CreateTable
CREATE TABLE "User" (
    "id" INTEGER NOT NULL,

    CONSTRAINT "User_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "_blocked" (
    "A" INTEGER NOT NULL,
    "B" INTEGER NOT NULL
);

-- CreateTable
CREATE TABLE "_friends" (
    "A" INTEGER NOT NULL,
    "B" INTEGER NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "Voice_voice_id_key" ON "Voice"("voice_id");

-- CreateIndex
CREATE UNIQUE INDEX "UserConfig_userId_key" ON "UserConfig"("userId");

-- CreateIndex
CREATE UNIQUE INDEX "User_id_key" ON "User"("id");

-- CreateIndex
CREATE UNIQUE INDEX "_blocked_AB_unique" ON "_blocked"("A", "B");

-- CreateIndex
CREATE INDEX "_blocked_B_index" ON "_blocked"("B");

-- CreateIndex
CREATE UNIQUE INDEX "_friends_AB_unique" ON "_friends"("A", "B");

-- CreateIndex
CREATE INDEX "_friends_B_index" ON "_friends"("B");

-- AddForeignKey
ALTER TABLE "VoiceAccess" ADD CONSTRAINT "VoiceAccess_voice_id_fkey" FOREIGN KEY ("voice_id") REFERENCES "Voice"("voice_id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "_blocked" ADD CONSTRAINT "_blocked_A_fkey" FOREIGN KEY ("A") REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "_blocked" ADD CONSTRAINT "_blocked_B_fkey" FOREIGN KEY ("B") REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "_friends" ADD CONSTRAINT "_friends_A_fkey" FOREIGN KEY ("A") REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "_friends" ADD CONSTRAINT "_friends_B_fkey" FOREIGN KEY ("B") REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE;
