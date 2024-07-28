package main

import (
	"context"
	"encoding/json"
	"fmt"
	"os"

	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-lambda-go/lambda"
	"github.com/aws/aws-sdk-go-v2/aws"
	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/service/dynamodb"
	"github.com/aws/aws-sdk-go-v2/service/dynamodb/types"
	"github.com/google/uuid"
)

type LeaderboardEvent struct {
	Name string `json:"name"`
	Score float64 `json:"score"`
}

type LeaderboardResponse struct {
	Id string `json:"id"`
	Name string `json:"name"`
	Score float64 `json:"score"`
}

func HandleRequest(ctx context.Context, req events.APIGatewayProxyRequest) (*events.APIGatewayProxyResponse, error) {
	cfg, err := config.LoadDefaultConfig(ctx)
	if err != nil {
		return nil, err
	}

	if req.Body == "" {
		return &events.APIGatewayProxyResponse{
			StatusCode: 400,
			Body: "Bad Request",
		}, nil
	}

	event := LeaderboardEvent{}
	json.Unmarshal([]byte(req.Body), &event)
	

	if event.Name == "" {
		return nil, fmt.Errorf("name is required")
	}

	if event.Score == 0 {
		return nil, fmt.Errorf("score is required")
	}

	dynamo := dynamodb.NewFromConfig(cfg)

	id := uuid.New()

	dynamo.PutItem(ctx, &dynamodb.PutItemInput{
		TableName: aws.String(os.Getenv("TABLE_NAME")),
		Item: map[string]types.AttributeValue{
			"id": &types.AttributeValueMemberS{Value: id.String()},
			"name": &types.AttributeValueMemberS{Value: event.Name},
			"score": &types.AttributeValueMemberN{Value: fmt.Sprintf("%f", event.Score)},
		},
	})

	body, err := json.Marshal(LeaderboardResponse{
		Id: id.String(),
		Name: event.Name,
		Score: event.Score,
	})
	if err != nil {
		return nil, err
	}

	return &events.APIGatewayProxyResponse{
		Headers: map[string]string{
			"Access-Control-Allow-Origin": "*",
			},
		StatusCode: 200,
		Body: string(body),
	}, nil
}

func main() {
	lambda.Start(HandleRequest)
}