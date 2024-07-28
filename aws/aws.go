package main

import (
	"github.com/aws/aws-cdk-go/awscdk/v2"
	"github.com/aws/aws-cdk-go/awscdk/v2/awsapigateway"
	"github.com/aws/aws-cdk-go/awscdk/v2/awsdynamodb"
	"github.com/aws/aws-cdk-go/awscdklambdagoalpha/v2"
	"github.com/aws/constructs-go/constructs/v10"
	"github.com/aws/jsii-runtime-go"
)

type ClockeryStackProps struct {
	awscdk.StackProps
}

func NewAwsStack(scope constructs.Construct, id string, props *ClockeryStackProps) awscdk.Stack {
	var sprops awscdk.StackProps
	if props != nil {
		sprops = props.StackProps
	}
	stack := awscdk.NewStack(scope, &id, &sprops)

	postLambda := awscdklambdagoalpha.NewGoFunction(stack, jsii.String("ClockeryLeaderboardPostFunction"), &awscdklambdagoalpha.GoFunctionProps{
		Entry: jsii.String("lambda/leaderboard/post/post.go"),
		Environment: &map[string]*string{
			"TABLE_NAME": jsii.String("ClockeryLeaderboard"),
		},
	})

	getLambda := awscdklambdagoalpha.NewGoFunction(stack, jsii.String("ClockeryLeaderboardGetFunction"), &awscdklambdagoalpha.GoFunctionProps{
		Entry: jsii.String("lambda/leaderboard/get-all/get.go"),
		Environment: &map[string]*string{
			"TABLE_NAME": jsii.String("ClockeryLeaderboard"),
		},
	})

	dynamo := awsdynamodb.NewTable(stack, jsii.String("ClockeryLeaderboard"), &awsdynamodb.TableProps {
		PartitionKey: &awsdynamodb.Attribute{ Name: jsii.String("id"), Type: awsdynamodb.AttributeType_STRING },
		TableName: jsii.String("ClockeryLeaderboard"),
	})
	dynamo.GrantFullAccess(postLambda)
	dynamo.GrantFullAccess(getLambda)

	api := awsapigateway.NewRestApi(stack, jsii.String("ClockeryLeaderboardApi"), &awsapigateway.RestApiProps{
		RestApiName: jsii.String("ClockeryLeaderboardApi"),
		DefaultCorsPreflightOptions: &awsapigateway.CorsOptions{
			AllowOrigins: awsapigateway.Cors_ALL_ORIGINS(),
			AllowMethods: awsapigateway.Cors_ALL_METHODS(),
			},
	})

	resource := api.Root().AddResource(
		jsii.String("leaderboard"), 
		&awsapigateway.ResourceOptions{
			DefaultCorsPreflightOptions: &awsapigateway.CorsOptions{
				AllowOrigins: awsapigateway.Cors_ALL_ORIGINS(),
				AllowMethods: awsapigateway.Cors_ALL_METHODS(),
			},
		},
	)
	
	resource.AddMethod(
		jsii.String("GET"),
		awsapigateway.NewLambdaIntegration(getLambda, &awsapigateway.LambdaIntegrationOptions{}),
		nil,
	)

	resource.AddMethod(
		jsii.String("POST"),
		awsapigateway.NewLambdaIntegration(postLambda, &awsapigateway.LambdaIntegrationOptions{}),
		nil,
	)

	return stack
}

func main() {
	defer jsii.Close()

	app := awscdk.NewApp(nil)

	NewAwsStack(app, "ClockeryStack", &ClockeryStackProps{
		awscdk.StackProps{
			Env: env(),
		},
	})

	app.Synth(nil)
}

func env() *awscdk.Environment {
	return nil
}
