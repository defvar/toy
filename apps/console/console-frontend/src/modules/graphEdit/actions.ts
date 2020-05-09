import { ServiceState } from './types';

export interface GetServices {
    type: "GetServices",
    payload: {
        items: { [key: string]: ServiceState }
    }
}

export type Actions = GetServices;
