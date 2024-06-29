import * as anchor from "@coral-xyz/anchor";

import { Ctx } from "./ctx";

type Balance = number | anchor.BN | bigint;
export namespace CheckCtx {

    export async function config(ctx: Ctx) {
        const config = await ctx.program.account.configAccount.fetch(ctx.configAccount);
        return config;
    }

    export async function season(ctx: Ctx) {
        const season = await ctx.program.account.seasonAccount.fetch(ctx.seasonAccount);
        return season;
    }

    export async function memeTeam(ctx: Ctx) {
        const memeTeam = await ctx.program.account.teamAccount.fetch(ctx.memeTeamAccount);
        return memeTeam;
    }

    export async function chainTeam(ctx: Ctx) {
        const chain = await ctx.program.account.teamAccount.fetch(ctx.chainTeamAccount);
        return chain;
    }

    export async function user( ctx: Ctx) {
        const user = await ctx.program.account.userAccount.fetch(ctx.user1Account);
        return user;
    }

    export async function getGameState(ctx: Ctx) {
        const gameState = await ctx.program.account.gameAccount.fetch(ctx.gameAccount);
        console.table(gameState.board);
    }
}