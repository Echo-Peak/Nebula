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
        config.set(propName, propValue);
        return new Response(`The prop "${propName}" was set successfully`);
      } catch (err: any) {
        return new Response(`Unable to set prop: ${err.message}`);
      }
      break;
    }
    default: {
      return new Response("");
    }
  }
}