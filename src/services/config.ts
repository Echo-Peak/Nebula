

const mockConfig = {
    prefences: {
        darkMode: false,
        cache: "1GB",
        excludsions: [

        ],
        buckets: []
    },
}
const _config = {
    get(sd: any) { },
    set(sdf: any, df: any) { }
}

export const config = {
    get<T>(propName: string): T {
        const val = _config.get(propName) as any;
        return val || '';
    },
    set<T>(propName: string, propValue: string | T) {
        _config.set(propName, propValue);
    }
}
