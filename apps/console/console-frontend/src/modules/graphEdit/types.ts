export interface GraphEditState {
    services: { [key: string]: ServiceState };
}

export interface ServiceState {
    name: string;
    description: string;
    out: number;
    in: number;
}
