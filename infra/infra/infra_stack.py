import os
from aws_cdk import (
    core as cdk,
    aws_lambda,
    aws_ecr
)


class InfraStack(cdk.Stack):

    def __init__(self, scope: cdk.Construct, construct_id: str, **kwargs) -> None:
        super().__init__(scope, construct_id, **kwargs)

        repository_name = "rust_lambda"

        ecr_image = aws_lambda.EcrImageCode.from_asset_image(
            directory=os.path.join(os.getcwd(), "../docker"),
            repository_name=repository_name)

        aws_lambda.Function(self,
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
