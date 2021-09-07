package io.parity.signer.screens

import android.widget.Toast
import androidx.camera.core.CameraSelector
import androidx.camera.core.ImageAnalysis
import androidx.camera.core.Preview
import androidx.camera.lifecycle.ProcessCameraProvider
import androidx.camera.view.PreviewView
import androidx.compose.animation.expandHorizontally
import androidx.compose.foundation.Image
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.material.Button
import androidx.compose.material.ButtonDefaults
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.viewinterop.AndroidView
import androidx.core.content.ContextCompat
import io.parity.signer.MainActivity
import io.parity.signer.models.SignerDataModel

/**
 * This is a simple screen with a single button that
 * triggers transaction sequence starting with camera
 */
@Composable
fun HomeScreen(signerDataModel: SignerDataModel, navToTransaction: () -> Unit) {
	val lifecycleOwner = LocalLifecycleOwner.current
	val context = LocalContext.current
	val cameraProviderFuture =
		remember { ProcessCameraProvider.getInstance(context) }

	Box(
		modifier = Modifier
			.clickable(onClick = navToTransaction)
			.fillMaxSize()
	) {
		//TODO: add proper camera image
		AndroidView(
			factory = { context ->
				val executor = ContextCompat.getMainExecutor(context)
				val previewView = PreviewView(context)
				cameraProviderFuture.addListener({
					val cameraProvider = cameraProviderFuture.get()

					val preview = Preview.Builder().build().also {
						it.setSurfaceProvider(previewView.surfaceProvider)
					}

					val cameraSelector = CameraSelector.Builder()
						.requireLensFacing(CameraSelector.LENS_FACING_BACK)
						.build()

					val imageAnalysis = ImageAnalysis.Builder()
						.setBackpressureStrategy(ImageAnalysis.STRATEGY_KEEP_ONLY_LATEST)
						.build()
						.apply {
							setAnalyzer(executor, ImageAnalysis.Analyzer {  })
						}

					cameraProvider.unbindAll()
					cameraProvider.bindToLifecycle(
						lifecycleOwner,
						cameraSelector,
						preview
					)
				}, executor)
				previewView
			},
			modifier = Modifier.fillMaxSize()
		)

	}
}

