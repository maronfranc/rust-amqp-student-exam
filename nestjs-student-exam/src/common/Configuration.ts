import { loadEnv } from "./loadEnv";

loadEnv();

export const rabbitmqKeys = {
    username: process.env.RABBIT_USERNAME ?? "",
    password: process.env.RABBIT_PASSWORD ?? "",
    host: process.env.RABBIT_HOST ?? "",
    port: process.env.RABBIT_PORT ?? "",
    vhost: process.env.RABBIT_VHOST ?? ""
} as const;