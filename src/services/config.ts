import Conf from 'conf';

const mockConfig = {
    prefences:{
        darkMode: false,
        cache: "1GB",
        excludsions:[

        ],
        buckets:[]
    },
}
const _config = new Conf();

export const config = {
    get(propName: string): string {
        const val = _config.get(propName) as string;
        if(val && val.length){
            return val;
        } 
        return '';
    },
    set(propName: string, propValue: string) {
        _config.set(propName, propValue);
    }
}
