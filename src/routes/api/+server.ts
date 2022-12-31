import { error } from '@sveltejs/kit';
import { getConfig } from '../../services/config';

/** @type {import('./$types').RequestHandler} */
export async function POST(evt: any) {
  const data = await evt.request.json();
  switch (data.name) {
    case "config.set": {
      const propName = data.data.propName;
      const propValue = data.data.propValue;
      const config = await getConfig();
      try {
        await config.set(propName, propValue);
        return new Response(`The prop "${propName}" was set successfully`);
      } catch (err: any) {
        return new Response(`Unable to set prop: ${err.message}`);
      }
    }
    case "config.get": {
      const propName = data.data.propName;
      const config = await getConfig();
      try {
        const result = config.get(propName);
        console.log('----->>>', result)
        return new Response(JSON.stringify({ name: data.name, data: result }));
      } catch (err: any) {
        return new Response(`Unable to set prop: ${err.message}`);
      }
    }
    default: {
      return new Response("");
    }
  }
}