import * as dotenv from "dotenv";

const dotenvs = {
    dev: ".env.dev",
};

export enum EnvValueType {
    Build = "BUILD"
}

export const getEnvValue = (value: EnvValueType): string => process.env[value] ?? "";

export const loadEnv = (): void => {
    const envBuild = getEnvValue(EnvValueType.Build);

    dotenv.config({
        path: dotenvs[envBuild]
    });

};
