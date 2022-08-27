<div align="center">
<h1>AZCON</h1>
<h2>Azure Connect: AKS</h2>
<p>By Abe Choi</p>
</div>

<p>
A CLI Tool for connecting to Azure clusters via kubectl config, and quickly finding errors within pods, components, and helm releases.
</p>

1.  [Run App](#run-app)
2.  [Create Command](#create-command)
3.  [How to Use](#how-to-use)


## Run App

There are 2 binary executables available:

- MacOS Terminal `/target/debug/azcon`, use `./azcon` to execute
- Windows Powershell `/target/x86_64-pc-windows-gnu/debug/azcon.exe`, use `.\azcon.exe` to execute.

To use all the features, install Azure CLI, Kubernetes, and Helm on the same environment as the azcon. This is important because you must be logged into Azure using `az login`, and there must be `kubectl config` available to fetch cluster data from.


### Create Command

To create a command for MacOS:

1. Copy azcon, `/target/debug/azcon`, into `/usr/local/bin`.
```
cp /target/debug/azcon /usr/local/bin
```

2. Check if path exists.
```
echo $PATH
```

if `/usr/local/bin` string is not found in the output, create it.
```
export PATH=$PATH:/usr/local/bin
```

Repeat step 2. and verify `/usr/local/bin` is inside the PATH. If so, restart the terminal.

3. Call `azcon` command from directory.
```
azcon
```

If you do not have the `servicetitan/scripts` repo, clone it.
```
# change directory to ~/ServiceTitan
cd ~/ServiceTitan

# clone repo inside ~/ServiceTitan
git clone https://github.com/servicetitan/scripts.git
```


### How to Use

- Upon executing, you should see which cluster/context you are connected to at the top of the menu. This is displayed using command: `kubectl config current-context`. If you see "you are currently connected to NONE", use command: `kubectl config current-context` to ensure `kubectl config` file is available in your environment, credentials are checked using `az login`, and that there is a cluster that is currently connected.

1. Connect to cluser - displays a numered list of every cluster cached inside `kubectl config`, entering a number will connect to that cluster.

2. Namespace report - prompts for a namespace inside the cluster, which outputs only lines with errors for `get deployments`, `get pods`, `get hr` for the namespace. Then a `Components Report` will display more choices to either for `helm history` for components or `get events` for pods.

NOTE: This does not delete the cluster from the Azure Portal!
3. Delete a cluster - displays a numered list of every cluster cached inside `kubectl config`, entering a number will remove the cluster from the `kubectl config` file.
