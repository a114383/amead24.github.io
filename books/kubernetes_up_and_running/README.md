# Kubernetes Up & Running

Hightower, Kelsey. 2017.  Kubernetes Up & Running.

## chapter 1 

## chapter 2

Here are the steps I needed to get the docker file to build for the rest of the examples:
```
sudo apt install golang-go
export GOPATH=$HOME/go

sudo apt install docker.io
sudo usermod -a -G docker $USER

git clone https://github.com/kubernetes-up-and-running/kuard
cd kuard
make build DOCKER_MOUNT_OPTION=
mkdir ../bin
cp bin/1/amd64/kuard ../bin/

cd ..
docker build -t kuard-amd64:1 .
```

## chapter 3

Kubernetes on AWS: https://medium.com/containermind/how-to-create-a-kubernetes-cluster-on-aws-in-few-minutes-89dda10354f4

Step 8.5 `ssh-keygen`

1. Components that make up a Kubernetes cluster:
`kubectl get componentstatuses`

`controller-manager` is responsible for controllers that regulate behavior, while `scheduler` is responsible for placing pods on different nodes.  Lastly, the `etcd` is storage for the cluster where all API objects are stored.

2. Check health and status of nodes:
`kubectl get nodes`

`master` nodes usually don't get work, instead oversee the operations, while `worker` nodes run the application containers.

3. Finally getting details on nodes:
`kubectl describe nodes node-1`

## chapter 4

Namespaces organize objects, like a folder.  Pass `--namespace` flag with `kubectl` to change it.

Contexts is the default namespace, `config set-context` and `config use-context`, and can be useful when multiple clusters exist.

Everything is an API object retrieved using RESTful calls.  Thus using `kubectl get <resource>` or just `kubectl get` will show many of the available commands.

## chapter 5

Pods = atmoic unit inside a cluster.  Best to ask yourself if containers still work if they land on different machines.

Added pods using the `kubectl apply -f <yaml file>`.  See the files starting with `kuard-*`.  Using .yaml configuration files you can setup liveness checks (requests are returning not just locked), readiness checks (service is running before sending requests), and resource limits/minimums.

Important to point out mounting of persistant data for use cases:
1. cache - see `emptyDir`
2. persistent - see kubernetes storage protocols ex: `nfs`
3. host filesystem - see `hostDir`

## chapter 6

Labels - identifying information, useful for query potentional
Annotations - non-identifying information, useful for sending data to external tools and libraries.

## chapter 7

Deployments - Another Kubernetes object, such as a Pod, however deployments take a template file and make sure that a pod is uptodate given the `spec` declaration.

Service Discovery - `kubectl expose deployment <name>`
DNS naming : <service name>.<namespace>.svc.cluster.local
With this you can refer to a service in the same namespace using name, across namespaces using name.namespace, and the full name from anywhere.

cleanup: `kubectl delete services,deployments -l app`

## chapter 8

ReplicaSets are decoupled from pods, they use labels to query on which pods they should be managing.  Same as deployments, and pods, ReplicaSets are setup using specification yaml files.  See: `ch8_rs.yaml`.

Scaling is easy using ReplicaSets, just don't forget to edit the appropriate text file:
`kubectl scale kuard-rs --replicas=4`

Better, albit slower, would be to edit the .yaml file and re-`-f apply` to broadcast changes.

Finally using `Horizontal Pod Scaling (HPA)` ReplicaSets can scale horizontally, that is, partition more machines through more pods using `autoscaling`:
`kubectl autoscale rs kuard --min=2 --max=5 --cpu-percent=80`

Cleanup: `kubectl delete rs kuard-rs --cascade=false #to keep pods around`

## chapter 9

