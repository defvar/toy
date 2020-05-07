
export interface GraphListItemState {
    name: string,
    labels: string[],
    isActive: boolean,
    isLoading: boolean,
}

export interface GraphListState {
   items: { [key: string]: GraphListItemState }
}
