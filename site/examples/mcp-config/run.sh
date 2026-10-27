# The build replaces the build machine's path with an example path.
biomcp mcp-config | sed "s#$(command -v biomcp)#/home/you/.local/bin/biomcp#"
