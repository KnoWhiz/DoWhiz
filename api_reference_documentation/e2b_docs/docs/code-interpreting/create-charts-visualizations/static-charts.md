> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Static charts

Every time you run Python code with `runCode()` in JavaScript or `run_code()` method in Python, the code is executed in a headless Jupyter server inside the sandbox.

E2B automatically detects any plots created with Matplotlib and sends them back to the client as images encoded in the base64 format.
These images are directly accesible on the `result` items in the `execution.results` array.

Here's how to retrieve a static chart from the executed Python code that contains a Matplotlib plot.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'
  import fs from 'fs'

  const codeToRun = `
  import matplotlib.pyplot as plt

  plt.plot([1, 2, 3, 4])
  plt.ylabel('some numbers')
  plt.show()
  `
  const sandbox = await Sandbox.create()

  // Run the code inside the sandbox
  const execution = await sandbox.runCode(codeToRun)

   // There's only one result in this case - the plot displayed with `plt.show()`
  const firstResult = execution.results[0]

  if (firstResult.png) {
    // Save the png to a file. The png is in base64 format.
    fs.writeFileSync('chart.png', firstResult.png, { encoding: 'base64' })
    console.log('Chart saved as chart.png')
  }
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import base64
  from e2b_code_interpreter import Sandbox

  code_to_run = """
  import matplotlib.pyplot as plt

  plt.plot([1, 2, 3, 4])
  plt.ylabel('some numbers')
  plt.show()
  """

  sandbox = Sandbox.create()

  # Run the code inside the sandbox
  execution = sandbox.run_code(code_to_run)

  # There's only one result in this case - the plot displayed with `plt.show()`
  first_result = execution.results[0]

  if first_result.png:
    # Save the png to a file. The png is in base64 format.
    with open('chart.png', 'wb') as f:
      f.write(base64.b64decode(first_result.png))
    print('Chart saved as chart.png')
  ```
</CodeGroup>

The code in the variable `codeToRun`/`code_to_run` will produce this following plot that we're saving as `chart.png` file.

<Frame>
  <img src="https://mintcdn.com/e2b/bga6ifW6jAKoRCaG/images/static-chart.png?fit=max&auto=format&n=bga6ifW6jAKoRCaG&q=85&s=ac4d548c4f4b6ac4dbd945692173a6ed" data-og-width="567" width="567" data-og-height="413" height="413" data-path="images/static-chart.png" data-optimize="true" data-opv="3" srcset="https://mintcdn.com/e2b/bga6ifW6jAKoRCaG/images/static-chart.png?w=280&fit=max&auto=format&n=bga6ifW6jAKoRCaG&q=85&s=30d54028c818ff3de2753827d77d23bd 280w, https://mintcdn.com/e2b/bga6ifW6jAKoRCaG/images/static-chart.png?w=560&fit=max&auto=format&n=bga6ifW6jAKoRCaG&q=85&s=a86c3b292ca3bc18e5f24de207c2b978 560w, https://mintcdn.com/e2b/bga6ifW6jAKoRCaG/images/static-chart.png?w=840&fit=max&auto=format&n=bga6ifW6jAKoRCaG&q=85&s=33b08ac673ab7d7c691b9430d3d9e86a 840w, https://mintcdn.com/e2b/bga6ifW6jAKoRCaG/images/static-chart.png?w=1100&fit=max&auto=format&n=bga6ifW6jAKoRCaG&q=85&s=5e0904b06051a986134603a9184701d1 1100w, https://mintcdn.com/e2b/bga6ifW6jAKoRCaG/images/static-chart.png?w=1650&fit=max&auto=format&n=bga6ifW6jAKoRCaG&q=85&s=d8b43cc4afc014ccd6c8dee083e058b0 1650w, https://mintcdn.com/e2b/bga6ifW6jAKoRCaG/images/static-chart.png?w=2500&fit=max&auto=format&n=bga6ifW6jAKoRCaG&q=85&s=ff3d6b8778b16961d8f7b1c978f846d0 2500w" />
</Frame>
