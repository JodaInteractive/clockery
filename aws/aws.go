package main

import (
	cdk "github.com/aws/aws-cdk-go/awscdk/v2"
	apigateway "github.com/aws/aws-cdk-go/awscdk/v2/awsapigateway"
	dynamo "github.com/aws/aws-cdk-go/awscdk/v2/awsdynamodb"
	lambda "github.com/aws/aws-cdk-go/awscdklambdagoalpha/v2"
	"github.com/aws/constructs-go/constructs/v10"
	"github.com/aws/jsii-runtime-go"
)

type ClockeryStackProps struct {
	cdk.StackProps
}

func NewAwsStack(scope constructs.Construct, id string, props *ClockeryStackProps) cdk.Stack {
	var sprops cdk.StackProps
	if props != nil {
		sprops = props.StackProps
	}
	stack := cdk.NewStack(scope, &id, &sprops)

	postLambda := lambda.NewGoFunction(stack, jsii.String("ClockeryLeaderboardPostFunction"), &lambda.GoFunctionProps{
		Entry: jsii.String("lambda/leaderboard/post/post.go"),
		Environment: &map[string]*string{
			"TABLE_NAME": jsii.String("ClockeryLeaderboard"),
		},
	})

	getLambda := lambda.NewGoFunction(stack, jsii.String("ClockeryLeaderboardGetFunction"), &lambda.GoFunctionProps{
		Entry: jsii.String("lambda/leaderboard/get-all/get.go"),
		Environment: &map[string]*string{
			"TABLE_NAME": jsii.String("ClockeryLeaderboard"),
		},
	})

	dynamo := dynamo.NewTable(stack, jsii.String("ClockeryLeaderboard"), &dynamo.TableProps{
		PartitionKey: &dynamo.Attribute{Name: jsii.String("id"), Type: dynamo.AttributeType_STRING},
		TableName:    jsii.String("ClockeryLeaderboard"),
	})
	dynamo.GrantFullAccess(postLambda)
	dynamo.GrantFullAccess(getLambda)

	api := apigateway.NewRestApi(stack, jsii.String("ClockeryLeaderboardApi"), &apigateway.RestApiProps{
		RestApiName: jsii.String("ClockeryLeaderboardApi"),
		DefaultCorsPreflightOptions: &apigateway.CorsOptions{
			AllowOrigins: apigateway.Cors_ALL_ORIGINS(),
			AllowMethods: apigateway.Cors_ALL_METHODS(),
		},
	})

	resource := api.Root().AddResource(
		jsii.String("leaderboard"),
		&apigateway.ResourceOptions{
			DefaultCorsPreflightOptions: &apigateway.CorsOptions{
				AllowOrigins: apigateway.Cors_ALL_ORIGINS(),
				AllowMethods: apigateway.Cors_ALL_METHODS(),
			},
		},
	)

	resource.AddMethod(
		jsii.String("GET"),
		apigateway.NewLambdaIntegration(getLambda, &apigateway.LambdaIntegrationOptions{}),
		nil,
	)

	resource.AddMethod(
		jsii.String("POST"),
		apigateway.NewLambdaIntegration(postLambda, &apigateway.LambdaIntegrationOptions{}),
		nil,
	)

	return stack
}

func main() {
	defer jsii.Close()

	app := cdk.NewApp(nil)

	NewAwsStack(app, "ClockeryStack", &ClockeryStackProps{
		cdk.StackProps{
			Env: env(),
		},
	})

	app.Synth(nil)
}

func env() *cdk.Environment {
	return nil
}
