goProject = "github.com/bozaro/tech-db-homework"

node ('linux') {
	stage ('Checkout') {
		checkout([
			$class: 'GitSCM',
			branches: scm.branches,
			doGenerateSubmoduleConfigurations: scm.doGenerateSubmoduleConfigurations,
			extensions: scm.extensions + [
				[$class: 'CleanCheckout'],
				[$class: 'RelativeTargetDirectory', relativeTargetDir: "src/$goProject"],
				[$class: 'SubmoduleOption', disableSubmodules: false, recursiveSubmodules: false]
			],
			userRemoteConfigs: scm.userRemoteConfigs
		])
	}
	stage ('Generate') {
		sh """
export GOPATH="\$PWD"
export PATH="\$GOPATH/bin:\$PATH"
go get -u github.com/jteeuwen/go-bindata/...
cd src/$goProject
go generate -x -n 2>&1 | sed -e 's/\\s-debug\\s/ /g' | bash
"""
	}
	stage ('Build') {
		sh """
export GOPATH="\$PWD"
cd src/$goProject
go build
"""
	}
	stage ('Test') {
		sh """
export GOPATH="\$PWD"
cd src/$goProject
go test
"""
	}
}
