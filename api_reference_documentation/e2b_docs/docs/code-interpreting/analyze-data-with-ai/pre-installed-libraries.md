> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Pre-installed libraries

export const Requirements = () => {
  const {useEffect, useState} = React;
  const [lines, setLines] = useState([]);
  useEffect(() => {
    const parseRequirement = line => {
      const commentIndex = line.indexOf("#");
      if (commentIndex !== -1) {
        line = line.substring(0, commentIndex);
      }
      line = line.trim();
      if (!line) return null;
      if (line.startsWith("-") || line.includes("@")) return null;
      const match = line.match(/^([a-zA-Z0-9_.-]+)(\[[^\]]*\])?(.*)$/);
      if (!match) return null;
      let name = match[1];
      const extras = match[2];
      let versionPart = match[3] || "";
      const markerIndex = versionPart.indexOf(";");
      if (markerIndex !== -1) {
        versionPart = versionPart.substring(0, markerIndex);
      }
      versionPart = versionPart.trim();
      let version = "";
      if (versionPart) {
        const versionSpecs = versionPart.split(",").map(s => s.trim());
        const parsedSpecs = [];
        for (const spec of versionSpecs) {
          const specMatch = spec.match(/^(===|==|!=|~=|>=|<=|>|<)(.*)$/);
          if (specMatch) {
            const operator = specMatch[1];
            const ver = specMatch[2].trim();
            if (ver) {
              if (operator === "==") {
                version = ver;
                break;
              }
              parsedSpecs.push({
                operator,
                version: ver
              });
            }
          }
        }
        if (!version && parsedSpecs.length > 0) {
          version = parsedSpecs[0].version;
        }
      }
      if (extras) {
        name = name + extras;
      }
      return {
        name,
        version
      };
    };
    const load = async () => {
      const res = await fetch("https://raw.githubusercontent.com/e2b-dev/code-interpreter/refs/heads/main/template/requirements.txt");
      const text = await res.text();
      const packages = text.split("\n").map(parseRequirement).filter(Boolean);
      setLines(packages);
    };
    load();
  }, []);
  return <div>
      <ul>
        {lines.map(pkg => <li key={pkg.name}>
            <code>{pkg.name}</code> ({pkg.version ? `v${pkg.version}` : "latest"})
          </li>)}
      </ul>
    </div>;
};

The sandbox comes with a <a href="https://github.com/e2b-dev/code-interpreter/blob/main/template/requirements.txt">set of pre-installed Python libraries</a> for data analysis but you can <a href="/docs/quickstart/install-custom-packages">install additional packages</a>

<Requirements />
