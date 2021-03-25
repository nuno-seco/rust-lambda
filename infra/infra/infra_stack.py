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

        ecr_image = self.create_or_update_ecr_image()
        handler = self.create_or_update_lambda_function(ecr_image)
        api = self.create_or_update_api()
        games = api.root.add_resource("games")
        game = games.add_resource("{id}")
        self.create_or_update_games_methods(handler, games)
        self.create_or_update_game_methods(handler, game)

    def create_or_update_api(self):
        return aws_apigateway.RestApi(self, "guessing_game",
                                      rest_api_name="Rust Guessing Game",
                                      description="This fronts the rust lambda guessing game.")
        return api

    def create_or_update_lambda_function(self, ecr_image):
        return aws_lambda.Function(self,
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
        return handler

    def create_or_update_ecr_image(self):
        return aws_lambda.EcrImageCode.from_asset_image(
            directory=os.path.join(os.getcwd(), "../docker"),
            repository_name="rust_lambda")

    def create_or_update_game_methods(self, handler, game):
        get_game_info = aws_apigateway.LambdaIntegration(handler,
                                                         proxy=False,
                                                         integration_responses=[
                                                             aws_apigateway.IntegrationResponse(
                                                                 status_code="200",
                                                                 response_templates={
                                                                     "application/json": self.response_template()
                                                                 }
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
                        )]
                        )

        make_guess = aws_apigateway.LambdaIntegration(handler,
                                                      proxy=False,
                                                      integration_responses=[
                                                          aws_apigateway.IntegrationResponse(
                                                              status_code="200",
                                                              response_templates={
                                                                  "application/json": self.response_template()
                                                              }
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

    def create_or_update_games_methods(self, handler, games):
        create_game = aws_apigateway.LambdaIntegration(handler,
                                                       proxy=False,
                                                       integration_responses=[
                                                           aws_apigateway.IntegrationResponse(
                                                               status_code="200",
                                                               response_templates={
                                                                   "application/json": self.response_template()
                                                               }
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

    def response_template(self):
        return textwrap.dedent(
            """        
            #set($stage = $context.stage)
            #set($path = $context.resourcePath.replaceAll("/\{[a-z]*\}", ""))
            #set($id = $input.json('$.id').replaceAll("^.|.$", ""))
            #set($guesses = $input.json('$.guesses'))
            {
              "game":
               {
                 "id" : "${id}",
                 "guesses" : ${guesses}
               },
              "links":
               {                                                                            
                 "status": 
                  {
                    "method": "GET",
                    "link": "/${stage}${path}/${id}"
                  },
                  "guess":
                   {
                     "method": "POST",
                     "link": "/${stage}${path}/${id}",
                     "body": 
                      {
                        "type": "object",
                        "properties": 
                         {
                            "guess": 
                             {
                               "type": "integer",
                               "minimum": 0,
                               "maximum": 10
                             }
                         }   
                      }    
                    }
                }
            }
        """)
