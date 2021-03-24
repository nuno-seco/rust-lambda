import os
import textwrap
from aws_cdk import (
    core as cdk,
    aws_lambda,
    aws_ecr,
    aws_apigateway
)


class InfraStack(cdk.Stack):

    def __init__(self, scope: cdk.Construct, construct_id: str, **kwargs) -> None:
        super().__init__(scope, construct_id, **kwargs)

        ecr_image = aws_lambda.EcrImageCode.from_asset_image(
            directory=os.path.join(os.getcwd(), "../docker"),
            repository_name="rust_lambda")

        handler = aws_lambda.Function(self,
                                      id="rust_lambda_chapter_2",
                                      description="Guessing game from chapter 2 in the Rust Book",
                                      code=ecr_image,
                                      handler=aws_lambda.Handler.FROM_IMAGE,
                                      runtime=aws_lambda.Runtime.FROM_IMAGE,
                                      function_name="rust_lambda_chapter_2",
                                      memory_size=128,
                                      reserved_concurrent_executions=10,
                                      timeout=cdk.Duration.seconds(1),
                                      )

        api = aws_apigateway.RestApi(self, "guessing_game",
                                     rest_api_name="Rust Guessing Game",
                                     description="This fronts the rust lambda guessing game.")

        games = api.root.add_resource("games")
        game = games.add_resource("{id}")

        create_game = aws_apigateway.LambdaIntegration(handler,
                                                       proxy=False,
                                                       integration_responses=[
                                                           aws_apigateway.IntegrationResponse(
                                                               status_code="200"
                                                           )
                                                       ],
                                                       request_templates={
                                                           "application/json": textwrap.dedent(
                                                               """
                                                                       {
                                                                       "kind": "gameRequested"
                                                                       }
                                                                       """)
                                                       }
                                                       )

        games.add_method("POST",
                         create_game,
                         method_responses=[aws_apigateway.MethodResponse(
                             status_code="200"
                         )])

        get_game_info = aws_apigateway.LambdaIntegration(handler,
                                                         proxy=False,
                                                         integration_responses=[
                                                             aws_apigateway.IntegrationResponse(
                                                                 status_code="200"
                                                             )
                                                         ],
                                                         request_templates={
                                                             "application/json": textwrap.dedent(
                                                                 """
                                                                       {
                                                                       "kind": "gameInfoRequested",
                                                                       "id": "$input.params('id')"
                                                                       }
                                                                       """)
                                                         }
                                                         )

        game.add_method("GET",
                        get_game_info,
                        method_responses=[aws_apigateway.MethodResponse(
                            status_code="200"
                        )])

        make_guess = aws_apigateway.LambdaIntegration(handler,
                                                      proxy=False,
                                                      integration_responses=[
                                                          aws_apigateway.IntegrationResponse(
                                                              status_code="200"
                                                          )
                                                      ],
                                                      request_templates={
                                                          "application/json": textwrap.dedent(
                                                              """
                                                                    {
                                                                    "kind": "guessSubmitted",
                                                                    "guess": $input.json('$.guess'),
                                                                    "id": "$input.params('id')"
                                                                    }
                                                                    """)
                                                      }
                                                      )

        game.add_method("POST",
                        make_guess,
                        method_responses=[aws_apigateway.MethodResponse(
                            status_code="200"
                        )])
