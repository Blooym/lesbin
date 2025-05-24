import type { APIConfigurationResponse } from './hooks.server';

declare global {
    namespace App {
        interface Locals {
            authenticationToken: string;
            apiConfig: APIConfigurationResponse;
        }
        interface Errors {
            message: string;
        }
    }
}

export {};
