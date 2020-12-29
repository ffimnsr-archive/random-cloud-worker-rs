addEventListener('fetch', event => {
  const request = event.request;
  const url = new URL(request.url);
  if (url.pathname.startsWith("/arr")) {
    event.respondWith(handleRequestArr(event.request));
  } else if (url.pathname.startsWith("/str")) {
    const key = url.pathname.split("/").pop();
    if (key !== "str") {
      event.respondWith(handleRequestStr(event.request, key));  
    } else {
      return new Response({status: 404, statusText: "Invalid URL"})
    }
  }
})

async function handleRequestArr(request) {
  const { run_arr } = wasm_bindgen;
  await wasm_bindgen(wasm);
  const runner = run_arr();
  return new Response(JSON.stringify(
      {
        data: runner,
      }
    ), {
    headers: {
      'Content-Type': 'application/json;charset=UTF-8',
    },
    status: 200,
  });
}

async function handleRequestStr(request, key) {
  const { run_str } = wasm_bindgen;
  await wasm_bindgen(wasm);

  const old_value = await RANDOM.get(key)
  if (old_value === null) {
    const runner = run_str();
    const new_value = JSON.stringify({  data: runner });
    await RANDOM.put(key, new_value, { expirationTtl: 600 })
    return new Response(new_value, {
      headers: {
        'Content-Type': 'application/json;charset=UTF-8',
      },
      status: 200,
    });
  }

  return new Response(old_value, {
    headers: {
      'Content-Type': 'application/json;charset=UTF-8',
    },
    status: 200,
  });
}
