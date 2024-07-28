package main

import (
	"context"
	"encoding/json"
	"os"
	"sort"
	"strconv"

	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-lambda-go/lambda"
	"github.com/aws/aws-sdk-go-v2/aws"
	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/service/dynamodb"
	"github.com/aws/aws-sdk-go-v2/service/dynamodb/types"
)

type LeaderboardRecord struct {
	Id string `json:"id"`
	Name string `json:"name"`
	Score float64 `json:"score"`
}

func HandleRequest(ctx context.Context) (*events.APIGatewayProxyResponse, error) {
	cfg, err := config.LoadDefaultConfig(ctx)
	if err != nil {
		return nil, err
	}

	dynamo := dynamodb.NewFromConfig(cfg)

	result, err := dynamo.Scan(ctx, &dynamodb.ScanInput{
		TableName: aws.String(os.Getenv("TABLE_NAME")),
	})
	if err != nil {
		return nil, err
	}

	var entries []LeaderboardRecord
	for _, item := range result.Items {
		score, err := strconv.ParseFloat(item["score"].(*types.AttributeValueMemberN).Value, 32)
		if err != nil {
			return nil, err
		}
		s := score

		entries = append(entries, LeaderboardRecord{
			Id: item["id"].(*types.AttributeValueMemberS).Value,
			Name: item["name"].(*types.AttributeValueMemberS).Value,
			Score: s,
		})
	}
	sort.Slice(entries, func(i, j int) bool {
		return entries[i].Score > entries[j].Score
	})

	body, err := json.Marshal(entries)
	if err != nil {
		return nil, err
	}

	return &events.APIGatewayProxyResponse{
		StatusCode: 200,
		Body: string(body),
	}, nil
}

func main() {
	lambda.Start(HandleRequest)
}