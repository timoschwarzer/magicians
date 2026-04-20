export namespace MG {
    export type WebSocketMessage = {
        type: "CurrentGame",
        game: Game,
    } | {
        type: "SetPlayerIndex",
        playerIndex: number | null,
    } | {
        type: "SetPlayerKey",
        key: string,
    }

    export type Player = {
        name: string,
        prediction: number | null,
        points: number,
        readyToContinue: boolean,
    }

    export type CardColor = "Green" | "Blue" | "Yellow" | "Red"

    export type GameState = {
        type: "Setup"
    } | {
        type: "Idle",
        round: number,
        firstPlayerIndex: number,
        lastPlayerIndex: number,
    } | {
        type: "Predicting",
        round: number,
        playerIndex: number,
        lastPlayerIndex: number,
        disallowedPrediction: number | null,
    } | {
        type: "Playing",
        round: number,
    } | {
        type: "DistributingPoints",
        round: number,
    }

    export type Game = {
        createdAt: number,
        players: Player[],
        trumpColor: CardColor | null,
        state: GameState,
    }

    export type CreateGameResponse = {
        gameId: string,
    }

    export type WebSocketCommand = {
        type: "Authenticate",
        key: string,
    } | {
        type: "Join",
        name: string,
    } | {
        type: "SignalReady",
        ready: boolean,
    } | {
        type: "SetTrumpColor",
        trumpColor: CardColor | null,
    } | {
        type: "SubmitPrediction",
        prediction: number,
    } | {
        type: "ChangePrediction",
        prediction: number,
    } | {
        type: "SubmitWinCount",
        winCount: number,
    }
}