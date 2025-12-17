import type { APIConfigurationResponse } from './hooks.server';

declare global {
    namespace App {
        interface Locals {
            apiConfig: APIConfigurationResponse;
        }
        interface Errors {
            message: string;
        }
    }
}

export {};
